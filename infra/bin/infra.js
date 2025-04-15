#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const cdk = require("aws-cdk-lib");
const infra_stack_1 = require("../lib/infra-stack");
/**
 * Initialize and validate environment configuration
 */
function getConfig() {
    const githubOrg = process.env.GITHUB_ORG;
    const githubRepo = process.env.GITHUB_REPO;
    if (!githubOrg || !githubRepo) {
        throw new Error('Both GITHUB_ORG and GITHUB_REPO environment variables are required');
    }
    // Validate organization name format
    if (!/^[a-zA-Z0-9-]+$/.test(githubOrg)) {
        throw new Error('GITHUB_ORG must contain only alphanumeric characters and hyphens');
    }
    // Validate repository name format
    if (!/^[a-zA-Z0-9-]+$/.test(githubRepo)) {
        throw new Error('GITHUB_REPO must contain only alphanumeric characters and hyphens');
    }
    return { githubOrg, githubRepo };
}
const app = new cdk.App();
try {
    const { githubOrg, githubRepo } = getConfig();
    new infra_stack_1.InfraStack(app, 'WishappStack', {
        githubOrg,
        githubRepo,
        env: {
            account: process.env.CDK_DEFAULT_ACCOUNT,
            region: process.env.CDK_DEFAULT_REGION
        }
    });
}
catch (error) {
    console.error('Configuration error:', error instanceof Error ? error.message : error);
    process.exit(1);
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5mcmEuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyJpbmZyYS50cyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiOzs7QUFDQSxtQ0FBbUM7QUFDbkMsb0RBQWdEO0FBRWhEOztHQUVHO0FBQ0gsU0FBUyxTQUFTO0lBQ2hCLE1BQU0sU0FBUyxHQUFHLE9BQU8sQ0FBQyxHQUFHLENBQUMsVUFBVSxDQUFDO0lBQ3pDLE1BQU0sVUFBVSxHQUFHLE9BQU8sQ0FBQyxHQUFHLENBQUMsV0FBVyxDQUFDO0lBRTNDLElBQUksQ0FBQyxTQUFTLElBQUksQ0FBQyxVQUFVLEVBQUU7UUFDN0IsTUFBTSxJQUFJLEtBQUssQ0FBQyxvRUFBb0UsQ0FBQyxDQUFDO0tBQ3ZGO0lBRUQsb0NBQW9DO0lBQ3BDLElBQUksQ0FBQyxpQkFBaUIsQ0FBQyxJQUFJLENBQUMsU0FBUyxDQUFDLEVBQUU7UUFDdEMsTUFBTSxJQUFJLEtBQUssQ0FBQyxrRUFBa0UsQ0FBQyxDQUFDO0tBQ3JGO0lBRUQsa0NBQWtDO0lBQ2xDLElBQUksQ0FBQyxpQkFBaUIsQ0FBQyxJQUFJLENBQUMsVUFBVSxDQUFDLEVBQUU7UUFDdkMsTUFBTSxJQUFJLEtBQUssQ0FBQyxtRUFBbUUsQ0FBQyxDQUFDO0tBQ3RGO0lBRUQsT0FBTyxFQUFDLFNBQVMsRUFBRSxVQUFVLEVBQUMsQ0FBQztBQUNqQyxDQUFDO0FBRUQsTUFBTSxHQUFHLEdBQUcsSUFBSSxHQUFHLENBQUMsR0FBRyxFQUFFLENBQUM7QUFFMUIsSUFBSTtJQUNGLE1BQU0sRUFBQyxTQUFTLEVBQUUsVUFBVSxFQUFDLEdBQUcsU0FBUyxFQUFFLENBQUM7SUFDNUMsSUFBSSx3QkFBVSxDQUFDLEdBQUcsRUFBRSxjQUFjLEVBQUU7UUFDbEMsU0FBUztRQUNULFVBQVU7UUFDVixHQUFHLEVBQUU7WUFDSCxPQUFPLEVBQUUsT0FBTyxDQUFDLEdBQUcsQ0FBQyxtQkFBbUI7WUFDeEMsTUFBTSxFQUFFLE9BQU8sQ0FBQyxHQUFHLENBQUMsa0JBQWtCO1NBQ3ZDO0tBQ0YsQ0FBQyxDQUFDO0NBQ0o7QUFBQyxPQUFPLEtBQUssRUFBRTtJQUNkLE9BQU8sQ0FBQyxLQUFLLENBQUMsc0JBQXNCLEVBQUUsS0FBSyxZQUFZLEtBQUssQ0FBQyxDQUFDLENBQUMsS0FBSyxDQUFDLE9BQU8sQ0FBQyxDQUFDLENBQUMsS0FBSyxDQUFDLENBQUM7SUFDdEYsT0FBTyxDQUFDLElBQUksQ0FBQyxDQUFDLENBQUMsQ0FBQztDQUNqQiIsInNvdXJjZXNDb250ZW50IjpbIiMhL3Vzci9iaW4vZW52IG5vZGVcbmltcG9ydCAqIGFzIGNkayBmcm9tICdhd3MtY2RrLWxpYic7XG5pbXBvcnQgeyBJbmZyYVN0YWNrIH0gZnJvbSAnLi4vbGliL2luZnJhLXN0YWNrJztcblxuLyoqXG4gKiBJbml0aWFsaXplIGFuZCB2YWxpZGF0ZSBlbnZpcm9ubWVudCBjb25maWd1cmF0aW9uXG4gKi9cbmZ1bmN0aW9uIGdldENvbmZpZygpOiB7Z2l0aHViT3JnOiBzdHJpbmcsIGdpdGh1YlJlcG86IHN0cmluZ30ge1xuICBjb25zdCBnaXRodWJPcmcgPSBwcm9jZXNzLmVudi5HSVRIVUJfT1JHO1xuICBjb25zdCBnaXRodWJSZXBvID0gcHJvY2Vzcy5lbnYuR0lUSFVCX1JFUE87XG4gIFxuICBpZiAoIWdpdGh1Yk9yZyB8fCAhZ2l0aHViUmVwbykge1xuICAgIHRocm93IG5ldyBFcnJvcignQm90aCBHSVRIVUJfT1JHIGFuZCBHSVRIVUJfUkVQTyBlbnZpcm9ubWVudCB2YXJpYWJsZXMgYXJlIHJlcXVpcmVkJyk7XG4gIH1cblxuICAvLyBWYWxpZGF0ZSBvcmdhbml6YXRpb24gbmFtZSBmb3JtYXRcbiAgaWYgKCEvXlthLXpBLVowLTktXSskLy50ZXN0KGdpdGh1Yk9yZykpIHtcbiAgICB0aHJvdyBuZXcgRXJyb3IoJ0dJVEhVQl9PUkcgbXVzdCBjb250YWluIG9ubHkgYWxwaGFudW1lcmljIGNoYXJhY3RlcnMgYW5kIGh5cGhlbnMnKTtcbiAgfVxuICBcbiAgLy8gVmFsaWRhdGUgcmVwb3NpdG9yeSBuYW1lIGZvcm1hdFxuICBpZiAoIS9eW2EtekEtWjAtOS1dKyQvLnRlc3QoZ2l0aHViUmVwbykpIHtcbiAgICB0aHJvdyBuZXcgRXJyb3IoJ0dJVEhVQl9SRVBPIG11c3QgY29udGFpbiBvbmx5IGFscGhhbnVtZXJpYyBjaGFyYWN0ZXJzIGFuZCBoeXBoZW5zJyk7XG4gIH1cbiAgXG4gIHJldHVybiB7Z2l0aHViT3JnLCBnaXRodWJSZXBvfTtcbn1cblxuY29uc3QgYXBwID0gbmV3IGNkay5BcHAoKTtcblxudHJ5IHtcbiAgY29uc3Qge2dpdGh1Yk9yZywgZ2l0aHViUmVwb30gPSBnZXRDb25maWcoKTtcbiAgbmV3IEluZnJhU3RhY2soYXBwLCAnV2lzaGFwcFN0YWNrJywge1xuICAgIGdpdGh1Yk9yZyxcbiAgICBnaXRodWJSZXBvLFxuICAgIGVudjoge1xuICAgICAgYWNjb3VudDogcHJvY2Vzcy5lbnYuQ0RLX0RFRkFVTFRfQUNDT1VOVCxcbiAgICAgIHJlZ2lvbjogcHJvY2Vzcy5lbnYuQ0RLX0RFRkFVTFRfUkVHSU9OXG4gICAgfVxuICB9KTtcbn0gY2F0Y2ggKGVycm9yKSB7XG4gIGNvbnNvbGUuZXJyb3IoJ0NvbmZpZ3VyYXRpb24gZXJyb3I6JywgZXJyb3IgaW5zdGFuY2VvZiBFcnJvciA/IGVycm9yLm1lc3NhZ2UgOiBlcnJvcik7XG4gIHByb2Nlc3MuZXhpdCgxKTtcbn0iXX0=