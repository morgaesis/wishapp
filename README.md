# WishApp

WishApp is a web application that allows users to create and manage wishlists. The application is built with Rust, consolidating both frontend and backend components into a single project. The infrastructure is designed to be cloud-native, utilizing AWS services within the free tier. CI/CD is managed using GitHub Actions.

## Project Structure
- **Frontend**: Contains the user interface components and logic.
- **Backend**: Handles API requests and business logic.

## Technologies Used
- **Rust**: For both frontend and backend development.
- **AWS**: Cloud services for hosting and infrastructure.
- **GitHub Actions**: For continuous integration and deployment.

## Setup Instructions
1. Clone the repository.
2. Install dependencies for both frontend and backend.
3. Configure AWS services.
4. Set up GitHub Actions for CI/CD.

## Development
- **Frontend**: Located in the `src/frontend` directory. To run the frontend, use the command `cargo leptos serve --features frontend -- --port 59350 --open`.
- **Backend**: Located in the `backend` directory.

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

## License
This project is licensed under the MIT License.