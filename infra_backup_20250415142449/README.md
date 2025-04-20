# WishApp Infrastructure

This directory contains AWS CDK code for provisioning WishApp's cloud infrastructure.

## Setup
1. Install AWS CDK for Rust: Follow the instructions at https://github.com/aws/aws-cdk-rust
2. Set up environment variables:
   - `AWS_ACCESS_KEY_ID`: Your AWS access key ID.
   - `AWS_SECRET_ACCESS_KEY`: Your AWS secret access key.
   - `AWS_REGION`: The AWS region to deploy to.
3. Deploy: `cargo run --bin deploy`

## Key Components
- GitHub OIDC integration for secure deployments
- IAM roles with least-privilege permissions
- Infrastructure-as-code using AWS CDK (Rust)

## Infrastructure Components
- ECS Fargate cluster for containerized application deployment
- Application Load Balancer for traffic distribution
- ECR repository for container images
- CloudWatch for logging and monitoring
- VPC with public and private subnets
- Security groups with least-privilege access

## Deployment Workflow
1. GitHub Actions authenticates via OIDC using `.github/workflows/deploy.yml`
2. Assumes IAM role with deployment permissions
3. Builds and pushes container image to ECR
4. Updates ECS service with new image
5. Validates deployment health

Required repository secrets:
- `AWS_ROLE_ARN`: ARN of the IAM role for GitHub Actions
- `AWS_REGION`: AWS region for deployment
- `ECR_REPOSITORY`: Name of the ECR repository