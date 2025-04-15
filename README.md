# WishApp

WishApp is a web application that allows users to create and manage wishlists

## Infrastructure
Cloud infrastructure is defined as code in the [infra/](infra/) directory using AWS CDK. Features include:
- Secure GitHub Actions deployment via OIDC
- Automated provisioning of AWS resources
- Infrastructure version control

## CI/CD
Managed using GitHub Actions with:
- Automated testing on PRs
- Secure deployments to AWS
- Infrastructure changes via pull requests

## License
This project is licensed under the MIT License.
