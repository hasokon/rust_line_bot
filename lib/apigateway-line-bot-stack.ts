import * as cdk from 'aws-cdk-lib';
import {Construct} from 'constructs';
import {Function} from 'aws-cdk-lib/aws-lambda';
import {RestApi, LambdaIntegration, Deployment, Stage} from 'aws-cdk-lib/aws-apigateway';

export class ApiGatewayLineBotStack extends cdk.Stack {
    constructor(scope: Construct, id: string, lambdaEventConsumer: Function, props?: cdk.StackProps) {
        super(scope, id, props);

        const api = new RestApi(this, 'line-bot-api', {
            restApiName: 'apigateway-line-bot',
            description: 'ApiGateway for LINE bot',
            deploy: false
        });

        const eventConsumer = api.root.addResource('events');
        eventConsumer.addMethod('POST', new LambdaIntegration(lambdaEventConsumer));

        const deployment = new Deployment(this, 'apigateway-line-bot-deployment', {
            api
        });

        const devStage = new Stage(this, 'apigateway-line-bot-stage', {
            deployment,
            stageName: 'dev'
        });
    }
}
