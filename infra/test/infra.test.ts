import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Template } from "aws-cdk-lib/assertions";
import * as Infra from "../lib/infra-stack";

class TestableInfraStack extends Infra.InfraStack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, {
      ...props,
      assetPath: "test-assets",
    });
  }
}

describe("Infrastructure Tests", () => {
  beforeEach(() => {
    jest.setTimeout(60000); // Increased timeout to 60 seconds
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  test("Lambda Function Created", async () => {
    console.log("Starting Lambda test...");
    const app = new cdk.App();
    const stack = new TestableInfraStack(app, "TestStack");

    console.log("Synthesizing stack...");
    app.synth();
    console.log("Synthesis complete");

    const template = Template.fromStack(stack);
    console.log("Making assertions...");

    template.hasResourceProperties("AWS::Lambda::Function", {
      Runtime: "provided.al2",
    });
    console.log("Lambda test completed");
  });

  test("API Gateway Created", async () => {
    console.log("Starting API Gateway test...");
    const app = new cdk.App();
    const stack = new TestableInfraStack(app, "TestStack");

    console.log("Synthesizing stack...");
    app.synth();
    console.log("Synthesis complete");

    const template = Template.fromStack(stack);
    console.log("Making assertions...");

    template.hasResourceProperties("AWS::ApiGateway::RestApi", {
      Name: "WishApi",
    });
    console.log("API Gateway test completed");
  });
});
