# Infrastructure Deployment

## Key Components
- AWS Lambda (Rust runtime)
- API Gateway REST API
- Automated PR environments

## Workflow
- PRs: Auto-deploy preview environments (pr-{number})
- Main branch: Manual approval required for production
- Closed PRs: Automatic cleanup

## Development
```sh
npm test   # Run infrastructure tests
npm build  # Compile CDK code
cdk synth  # Generate CloudFormation
```

## CI/CD
- Uses GitHub Actions with OIDC
- Production: api.example.com
- Previews: pr-{number}.api.example.com