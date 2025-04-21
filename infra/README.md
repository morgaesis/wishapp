# WishApp Deployment

## Deployment Goals
- **Main branch**: Auto-deploy to production with OIDC auth
- **PR branches**: Isolated stacks with auto-cleanup
- **Security**: GitHub OIDC with zero secrets
- **Cost control**: $10 budget alerts for PR environments

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

## Key Commands
```bash
# PR Environment (run in CI):
npx cdk deploy -c prNumber=$PR_NUMBER

# Production (run in CI):
npx cdk deploy

# Destroy PR Environment:
npx cdk destroy -c prNumber=$PR_NUMBER

# Local Development:
npm ci
npm test
npx cdk synth
AWS_PROFILE=dev npx cdk deploy
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
Run once per account:
```bash
aws iam create-open-id-connect-provider \
  --url https://token.actions.githubusercontent.com \
  --client-id-list sts.amazonaws.com \
  --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1
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
