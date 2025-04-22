import * as cdk from 'aws-cdk-lib';
import { Construct } from '@aws-cdk/core';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

class TestableInfraStack extends Infra.InfraStack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, {
      ...props,
      // Override asset path for testing
      assetPath: 'test-assets'
    });
  }
}

describe('Infrastructure Tests', () => {
  beforeEach(() => {
    jest.setTimeout(30000); // 30 second timeout
  });

  test('Lambda Function Created', async () => {
    console.log('Starting Lambda test...');
    const app = new cdk.App();
    const stack = new TestableInfraStack(app, 'TestStack');
    console.log('Stack created, synthesizing...');
    const template = Template.fromStack(stack);
    console.log('Template generated, making assertions...');

    template.hasResourceProperties('AWS::Lambda::Function', {
      Runtime: 'provided.al2'
    });
    console.log('Test completed');
  });

  test('API Gateway Created', async () => {
    console.log('Starting API Gateway test...');
    const app = new cdk.App();
    const stack = new TestableInfraStack(app, 'TestStack');
    console.log('Stack created, synthesizing...');
    const template = Template.fromStack(stack);
    console.log('Template generated, making assertions...');

    template.hasResourceProperties('AWS::ApiGateway::RestApi', {
      Name: 'WishApi'
    });
    console.log('Test completed');
  });
});
