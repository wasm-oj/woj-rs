# Rust API client for woj

You can interact with WASM OJ Wonderland through this API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.4
- Package version: 0.0.4
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `woj` and add the following to `Cargo.toml` under `[dependencies]`:

```
woj = { path = "./woj" }
```

## Documentation for API Endpoints

All URIs are relative to *https://woj.csie.cool*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthApi* | [**auth**](docs/AuthApi.md#auth) | **GET** /api/auth | 
*AuthApi* | [**login**](docs/AuthApi.md#login) | **POST** /api/auth/send | 
*AuthApi* | [**me**](docs/AuthApi.md#me) | **GET** /api/me | 
*ProblemApi* | [**get_problem**](docs/ProblemApi.md#get_problem) | **GET** /api/problem/{id} | 
*ProblemApi* | [**list_problems**](docs/ProblemApi.md#list_problems) | **GET** /api/problem | 
*SubmissionApi* | [**get_submission**](docs/SubmissionApi.md#get_submission) | **GET** /api/submission/{id} | 
*SubmissionApi* | [**list_submissions**](docs/SubmissionApi.md#list_submissions) | **GET** /api/submission | 
*SubmissionApi* | [**submit**](docs/SubmissionApi.md#submit) | **POST** /api/submission | 
*SystemApi* | [**system**](docs/SystemApi.md#system) | **GET** /api/sys | 


## Documentation For Models

 - [Auth200Response](docs/Auth200Response.md)
 - [GetProblem200Response](docs/GetProblem200Response.md)
 - [GetProblem200ResponseProblem](docs/GetProblem200ResponseProblem.md)
 - [GetProblem200ResponseProblemPolicyInner](docs/GetProblem200ResponseProblemPolicyInner.md)
 - [GetProblem200ResponseProblemTestcaseInner](docs/GetProblem200ResponseProblemTestcaseInner.md)
 - [GetSubmission200Response](docs/GetSubmission200Response.md)
 - [GetSubmission200ResponseSubmission](docs/GetSubmission200ResponseSubmission.md)
 - [ListProblems200Response](docs/ListProblems200Response.md)
 - [ListProblems200ResponseProblemsInner](docs/ListProblems200ResponseProblemsInner.md)
 - [ListSubmissions200Response](docs/ListSubmissions200Response.md)
 - [ListSubmissions200ResponseSubmissionsInner](docs/ListSubmissions200ResponseSubmissionsInner.md)
 - [Login200Response](docs/Login200Response.md)
 - [Login400Response](docs/Login400Response.md)
 - [LoginRequest](docs/LoginRequest.md)
 - [Me200Response](docs/Me200Response.md)
 - [Submit200Response](docs/Submit200Response.md)
 - [SubmitRequest](docs/SubmitRequest.md)
 - [System200Response](docs/System200Response.md)
 - [System200ResponseCfg](docs/System200ResponseCfg.md)
 - [System200ResponseStat](docs/System200ResponseStat.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

jacob@csie.cool
