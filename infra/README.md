# WishApp Deployment

## Deployment Goals
- **Main branch**: Auto-deploy to production with OIDC auth
- **PR branches**: Isolated stacks with auto-cleanup
- **Security**: GitHub OIDC with zero secrets
- **Cost control**: $10 budget alerts for PR environments

## Account Setup

### Key Accounts
- **Root (Management)**: Used for organization administration and bootstrapping deployments
- **Dev**: Development environments (bootstrapped from root account)
- **Prod**: Production environment (bootstrapped from root account)

### Bootstrap Commands
Run these authenticated to each **target account** (Dev/Prod):

```bash
# First authenticate to target account (Dev/Prod)
export AWS_PROFILE=target-account-admin  # Dev or Prod admin credentials

# Get target account ID
TARGET_ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)

# Set your root account ID (replace 123456789012)
ROOT_ACCOUNT_ID=123456789012

# Bootstrap target account
npx cdk bootstrap aws://${TARGET_ACCOUNT_ID}/${AWS_REGION} \
  --trust ${ROOT_ACCOUNT_ID} \
  --cloudformation-execution-policies "arn:aws:iam::aws:policy/AdministratorAccess" \
  -c githubRepo="morgaesis/wishapp"
```

Key notes:
- Must run authenticated to target account (Dev/Prod)
- --trust specifies root account ID
- githubRepo must match your GitHub org/repo
- Set AWS_REGION if not in your profile

### Deployment Contexts
- **Local Development**:
  ```bash
  AWS_PROFILE=dev-admin npx cdk deploy
  ```

- **CI/CD Pipeline**:
  ```bash
  # Uses OIDC credentials automatically
  : "${GITHUB_REPOSITORY:=morgaesis/wishapp}"  # Default if not in env
  
  # PR Environment
  npx cdk deploy -c prNumber=$PR_NUMBER -c githubRepo="${GITHUB_REPOSITORY}"
  
  # Production Deployment  
  npx cdk deploy -c githubRepo="${GITHUB_REPOSITORY}"
  ```
  
  Note: In GitHub Actions, `${GITHUB_REPOSITORY}` is automatically available. For local testing, you can:
  ```bash
  export GITHUB_REPOSITORY=morgaesis/wishapp
  ```

### Security Best Practices

#### IAM Role Configuration
```typescript
new iam.Role(this, 'GitHubActionsRole', {
  assumedBy: new iam.WebIdentityPrincipal(oidcProviderArn, {
    'StringEquals': {
      'token.actions.githubusercontent.com:aud': 'sts.amazonaws.com'
    },
    'StringLike': {
      'token.actions.githubusercontent.com:sub': [
        `repo:${githubRepo}:pull_request`,
        `repo:${githubRepo}:ref:refs/heads/main`
      ]
    }
  }),
  inlinePolicies: {
    deploymentAccess: new iam.PolicyDocument({
      statements: [
        new iam.PolicyStatement({
          effect: iam.Effect.ALLOW,
          actions: [
            'cloudformation:Describe*',
            'cloudformation:Get*',
            'cloudformation:List*',
            'cloudformation:CreateStack',
            'cloudformation:UpdateStack',
            'cloudformation:DeleteStack',
            's3:PutObject',
            's3:GetObject',
            's3:ListBucket',
            'iam:PassRole'
          ],
          resources: ['*'],
          conditions: {
            'StringEquals': {'aws:RequestedRegion': this.region},
            'ArnEquals': {'aws:ResourceTag/StackType': `stack-suffix`}
          }
        })
      ]
    })
  }
});
```

#### Key Principles
1. **Least Privilege**: Only grant necessary permissions
2. **Conditions**:
   - Restrict to specific GitHub repo
   - Limit to target AWS region
   - Scope via resource tags
3. **Audit**: Regularly review IAM usage
4. **Isolation**: Separate PR/prod environments
5. **Rotation**: Keep OIDC provider thumbprint updated

## Deployment Flow
```mermaid
%%{init: {'theme':'neutral'}}%%
graph LR
    subgraph PR[PR Flow]
        A[PR Opened] --> B[QA Tests]
        B --> C[Deploy PR Stack]
        C --> D[pr-* Resources]
        E[PR Closed] --> F[Destroy Stack]
    end
    
    subgraph Prod[Production Flow]
        G[Main Push] --> H[QA Tests]
        H --> I[Deploy Prod]
    end
    
    style PR fill:#f5f5ff,stroke:#333
    style Prod fill:#f5fffa,stroke:#333
```

## Architecture
```mermaid
graph TD
    GitHub -->|OIDC| AWS
    subgraph AWS
        PR_Stack[PR Stack] -->|pr-*| Resources
        Prod_Stack[Prod Stack] -->|prod| Resources
    end
    
    Resources --> Lambda
    Resources --> DynamoDB
    Resources --> S3
```

## Deployment Commands

### Initial Setup (One-Time)
```bash
# Install dependencies
npm ci

# Bootstrap AWS accounts (dev/prod)
npx cdk bootstrap aws://DEV-ACCOUNT/REGION \
  --trust ACCOUNT_ID \
  --cloudformation-execution-policies arn:aws:iam::aws:policy/AdministratorAccess

npx cdk bootstrap aws://PROD-ACCOUNT/REGION \
  --trust ACCOUNT_ID \
  --cloudformation-execution-policies arn:aws:iam::aws:policy/AdministratorAccess
```

### Manual OIDC Setup (Root Account)
```bash
# Create GitHub OIDC provider (one-time)
aws iam create-open-id-connect-provider \
  --url https://token.actions.githubusercontent.com \
  --client-id-list sts.amazonaws.com \
  --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1
```

### CI/CD Commands
```bash
# PR Environment Deployment
PR_NUMBER=123 REPO_NAME=morgaesis/wishapp
npx cdk deploy -c prNumber=$PR_NUMBER -c githubRepo=$REPO_NAME

# Production Deployment
REPO_NAME=morgaesis/wishapp
npx cdk deploy -c githubRepo=$REPO_NAME

# Destroy PR Stack
npx cdk destroy -c prNumber=$PR_NUMBER
```

### Local Development
```bash
# Synthesize CloudFormation template
npx cdk synth

# Deploy to dev account
AWS_PROFILE=dev npx cdk deploy

# Run tests
npm test

# View deployment outputs
npx cdk list
npx cdk deploy --outputs-file outputs.json
```

### Cost Monitoring
```bash
# Set budget alerts (run once per account)
aws budgets create-budget \
  --account-id $ACCOUNT_ID \
  --budget file://budgets/pr-budget.json \
  --notifications-with-subscribers file://budgets/notifications.json
```

## Deployment Flow

### 1. Manual Setup (Root Account)
```mermaid
%%{init: {'theme':'neutral'}}%%
graph TD
    A[Root] --> B[Create OIDC Provider]
    A --> C[Create github-actions Role]
    C -->|Trust Policy| D[Restrict to GitHub Repo]
    C -->|Permissions| E[PowerUserAccess]
    
``` 
### Complete Manual Setup
1. Create OIDC provider (one-time):
```bash
aws iam create-open-id-connect-provider \
  --url https://token.actions.githubusercontent.com \
  --client-id-list sts.amazonaws.com \
  --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1
```

2. Bootstrap CDK in each target account:
```bash
npx cdk bootstrap aws://ACCOUNT-NUMBER/REGION \
  --cloudformation-execution-policies arn:aws:iam::aws:policy/AdministratorAccess
```

### CI/CD Commands
PR Environment:
```bash
npx cdk deploy -c prNumber=$PR_NUMBER -c githubRepo=$REPO_NAME
```

Production:
```bash
npx cdk deploy -c githubRepo=$REPO_NAME
```

### Cleanup Commands
Destroy PR stack:
```bash
npx cdk destroy -c prNumber=$PR_NUMBER
```

### 2. PR Deployment
```mermaid
%%{init: {'theme':'neutral'}}%%
graph LR
    PR[PR Opened] --> B[Deploy PR Stack]
    B --> C[prDuring-PR Resources]
    PR --> D[Run Tests]
    E[PR Closed] --> F[Destroy Stack]
```
- Creates isolated `pr-*` resources
- Auto-destroys when PR closes

### 3. Production Deployment
```mermaid
%%{init: {'theme':'neutral'}}%%
graph LR
    Main[Push to Main] --> B[Run Tests]
    B --> C[Deploy Prod Stack]
    C --> D[Rollout Verification]
```
- Deploys to production account
- Requires main branch push
