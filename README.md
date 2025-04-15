# WishApp

WishApp is a web application that allows users to create and manage wishlists

## Infrastructure
Cloud-native infrastructure defined as code using AWS CDK, optimized for:
- **Cost efficiency**: Uses AWS Free Tier eligible services where possible
- **Security**: GitHub Actions deployment via OIDC (no long-lived credentials)
- **Reliability**: Automated provisioning with infrastructure-as-code
- **Scalability**: Designed for cloud-native patterns (containers, serverless)

## CI/CD
Managed using GitHub Actions with:
- Automated testing on PRs
- Secure deployments to AWS
- Infrastructure changes via pull requests

## License
This project is licensed under the MIT License.
