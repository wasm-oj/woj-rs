# \ProblemApi

All URIs are relative to *https://woj.csie.cool*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_problem**](ProblemApi.md#get_problem) | **GET** /api/problem/{id} | 
[**list_problems**](ProblemApi.md#list_problems) | **GET** /api/problem | 



## get_problem

> crate::models::GetProblem200Response get_problem(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetProblem200Response**](get_problem_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_problems

> crate::models::ListProblems200Response list_problems()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListProblems200Response**](list_problems_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

