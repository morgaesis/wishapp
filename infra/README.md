# WishApp Infrastructure

This directory contains AWS CDK code for provisioning WishApp's cloud infrastructure.

## Setup
1. Install CDK: `npm install -g aws-cdk`
2. Deploy: `cdk deploy`

## Key Components
- GitHub OIDC integration for secure deployments
- IAM roles with least-privilege permissions
- Infrastructure-as-code using AWS CDK (Rust)

## Deployment Workflow
1. GitHub Actions authenticates via OIDC
2. Assumes IAM role with necessary permissions
3. Deploys application using defined infrastructure