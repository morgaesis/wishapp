#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { InfraStack } from "../lib/infra-stack";

/**
 * Initialize and validate environment configuration
 */
function getConfig(): { githubOrg: string; githubRepo: string } {
  const githubOrg = process.env.GITHUB_ORG;
  const githubRepo = process.env.GITHUB_REPO;

  if (!githubOrg || !githubRepo) {
    throw new Error(
      "Both GITHUB_ORG and GITHUB_REPO environment variables are required"
    );
  }

  // Validate organization name format
  if (!/^[a-zA-Z0-9-]+$/.test(githubOrg)) {
    throw new Error(
      "GITHUB_ORG must contain only alphanumeric characters and hyphens"
    );
  }

  // Validate repository name format
  if (!/^[a-zA-Z0-9-]+$/.test(githubRepo)) {
    throw new Error(
      "GITHUB_REPO must contain only alphanumeric characters and hyphens"
    );
  }

  return { githubOrg, githubRepo };
}

const app = new cdk.App();

try {
  const { githubOrg, githubRepo } = getConfig();
  new InfraStack(app, "WishappStack", {
    githubOrg,
    githubRepo,
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: process.env.CDK_DEFAULT_REGION,
    },
  });
} catch (error) {
  console.error(
    "Configuration error:",
    error instanceof Error ? error.message : error
  );
  process.exit(1);
}
