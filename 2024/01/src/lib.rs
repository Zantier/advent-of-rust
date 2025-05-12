use std::io::{stdout, Write};

pub fn print_message(message: String) {
    // TODO: Implement the function
    // Should print `Message: <message>`
    write_message(message, &mut stdout()).unwrap();
}

fn write_message(message: String, writer: &mut impl Write) -> Result<(), std::io::Error> {
    let message = format!("Message: {message}\n");
    writer.write(message.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::write_message;

    #[test]
    fn test_write() {
        assert_eq!(write_message_to_string("hello"), "Message: hello\n");
        assert_eq!(write_message_to_string("Good message"), "Message: Good message\n");
    }

    fn write_message_to_string(message: &str) -> String {
        let mut buf = Vec::new();
        write_message(message.to_string(), &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
