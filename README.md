### XOR-MAILER

A mail library aimed at simplicity and ease of use.


##### Config file
Pass the config file to the binary. This config file contains the configuration of your mail server and is in the TOML format

```toml
sender = ["Example Company", "example@example.com"]
smtp_uri = "mail.example.com"
smtp_port = 587
smtp_username = "example@example.com"
smtp_password = "my_password"
```