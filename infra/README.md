# WishApp Infrastructure (AWS CDK)

This project contains the AWS infrastructure for WishApp, deployed using AWS CDK with TypeScript.

## Architecture Overview

- **Frontend**: S3 + CloudFront
- **Backend**: AWS Lambda functions
- **Database**: DynamoDB
- **CI/CD**: GitHub Actions with OIDC authentication

## Deployment Requirements

1. AWS credentials configured
2. Node.js 16+ installed
3. AWS CDK installed (`npm install -g aws-cdk`)

## Environment Variables

Required for deployment:

```bash
export GITHUB_ORG=your-github-org
export GITHUB_REPO=your-repo-name
```

## Deployment Commands

```bash
# Install dependencies
npm install

# Build the project
npm run build

# Synthesize CloudFormation template
npx cdk synth

# Deploy to AWS
npx cdk deploy

# Run tests
npm test
```

## Lambda Configuration

The infrastructure includes:

- API Gateway fronting Lambda functions
- Auto-scaling Lambda functions
- Environment variables for Lambda configuration
- Proper IAM permissions following least privilege

## GitHub Actions Integration

The stack creates:

- OIDC provider for GitHub Actions
- IAM role for deployments
- Limited permissions scoped to only required resources

## Maintenance

To update deployed stack:

```bash
# After making changes
npm run build
npx cdk deploy
```

## Cleanup

To delete all resources:

```bash
npx cdk destroy
```
