#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import {ApiGatewayLineBotStack} from '../lib/apigateway-line-bot-stack'
import {LambdaEventConsumerStack} from "../lib/lambda-event-consumer-stack";

const app = new cdk.App();

const lambdaEventConsumerStack = new LambdaEventConsumerStack(app, 'lambda-event-consumer-stack');
new ApiGatewayLineBotStack(app, 'apigateway-line-bot-stack', lambdaEventConsumerStack.lambdaFunction);

