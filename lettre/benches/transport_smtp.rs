#![feature(test)]

extern crate test;

use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, Envelope, SmtpClient};
use lettre::{EmailAddress, SendableEmail, Transport};

#[bench]
fn bench_simple_send(b: &mut test::Bencher) {
    let mut sender = SmtpClient::new("127.0.0.1:2525", ClientSecurity::None)
        .unwrap()
        .transport();
    b.iter(|| {
        let email = SendableEmail::new(
            Envelope::new(
                Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
                vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
            )
            .unwrap(),
            "id".to_string(),
            "Hello ß☺ example".to_string().into_bytes(),
        );
        let result = sender.send(email);
        assert!(result.is_ok());
    });
}

#[bench]
fn bench_reuse_send(b: &mut test::Bencher) {
    let mut sender = SmtpClient::new("127.0.0.1:2525", ClientSecurity::None)
        .unwrap()
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();
    b.iter(|| {
        let email = SendableEmail::new(
            Envelope::new(
                Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
                vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
            )
            .unwrap(),
            "id".to_string(),
            "Hello ß☺ example".to_string().into_bytes(),
        );
        let result = sender.send(email);
        assert!(result.is_ok());
    });
    sender.close()
}
