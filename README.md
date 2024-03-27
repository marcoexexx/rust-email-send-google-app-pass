**Speedy Inbox**

Speedy Inbox is a Rust-based email sender designed to simplify the process of sending emails with HTML templates. With a focus on efficiency and ease of use, Speedy Inbox allows users to send beautifully formatted emails quickly and effortlessly.

### Features:
- **Email Sending:** Send emails with HTML templates effortlessly.
- **Environment Variables:** Utilize environment variables for configuration:
  - `EMAIL_PASS`: Password for the email account.
  - `EMAIL_USERNAME`: Username for the email account.
  - `EMAIL_FROM`: Sender's email address.

### Usage:
1. Clone the repository:
   ```
   git clone https://github.com/marxoexexx/speedy-inbox.git
   ```
2. Set up environment variables:
   ```
   export EMAIL_PASS=your_email_password
   export EMAIL_USERNAME=your_email_username
   export EMAIL_FROM=your_email_address
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. Run the application:
   ```
   cargo run --release
   ```

### Contribution:
Contributions are welcome! Feel free to open issues and submit pull requests to help improve Speedy Inbox.

### License:
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

For more information, visit the [GitHub repository](https://github.com/marxoexexx/speedy-inbox).
