/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::InterceptorError;
use crate::type_erasure::TypeErasedBox;

pub type Input = TypeErasedBox;
pub type Output = TypeErasedBox;
pub type Error = TypeErasedBox;
pub type OutputOrError = Result<Output, Error>;

/// A container for the data currently available to an interceptor.
pub struct InterceptorContext<Request, Response> {
    input: Input,
    output_or_error: Option<OutputOrError>,
    request: Option<Request>,
    response: Option<Response>,
}

// TODO(interceptors) we could use types to ensure that people calling methods on interceptor context can't access
//     field that haven't been set yet.
impl<Request, Response> InterceptorContext<Request, Response> {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            output_or_error: None,
            request: None,
            response: None,
        }
    }

    /// Retrieve the modeled request for the operation being invoked.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// Retrieve the modeled request for the operation being invoked.
    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    /// Retrieve the transmittable request for the operation being invoked.
    /// This will only be available once request marshalling has completed.
    pub fn request(&self) -> Result<&Request, InterceptorError> {
        self.request
            .as_ref()
            .ok_or_else(InterceptorError::invalid_request_access)
    }

    /// Retrieve the transmittable request for the operation being invoked.
    /// This will only be available once request marshalling has completed.
    pub fn request_mut(&mut self) -> Result<&mut Request, InterceptorError> {
        self.request
            .as_mut()
            .ok_or_else(InterceptorError::invalid_request_access)
    }

    /// Retrieve the response to the transmittable response for the operation
    /// being invoked. This will only be available once transmission has
    /// completed.
    pub fn response(&self) -> Result<&Response, InterceptorError> {
        self.response
            .as_ref()
            .ok_or_else(InterceptorError::invalid_response_access)
    }

    /// Retrieve the response to the transmittable response for the operation
    /// being invoked. This will only be available once transmission has
    /// completed.
    pub fn response_mut(&mut self) -> Result<&mut Response, InterceptorError> {
        self.response
            .as_mut()
            .ok_or_else(InterceptorError::invalid_response_access)
    }

    /// Retrieve the response to the customer. This will only be available
    /// once the `response` has been unmarshalled or the attempt/execution has failed.
    pub fn output_or_error(&self) -> Result<Result<&Output, &Error>, InterceptorError> {
        self.output_or_error
            .as_ref()
            .ok_or_else(InterceptorError::invalid_modeled_response_access)
            .map(|res| res.as_ref())
    }

    /// Retrieve the response to the customer. This will only be available
    /// once the `response` has been unmarshalled or the
    /// attempt/execution has failed.
    pub fn output_or_error_mut(&mut self) -> Result<&mut Result<Output, Error>, InterceptorError> {
        self.output_or_error
            .as_mut()
            .ok_or_else(InterceptorError::invalid_modeled_response_access)
    }

    // There is no set_modeled_request method because that can only be set once, during context construction

    pub fn set_request(&mut self, request: Request) {
        if self.request.is_some() {
            panic!("Called set_request but a request was already set. This is a bug. Please report it.");
        }

        self.request = Some(request);
    }

    pub fn set_response(&mut self, response: Response) {
        if self.response.is_some() {
            panic!("Called set_response but a transmit_response was already set. This is a bug. Please report it.");
        }

        self.response = Some(response);
    }

    pub fn set_output_or_error(&mut self, output: Result<Output, Error>) {
        if self.output_or_error.is_some() {
            panic!(
                "Called set_output but an output was already set. This is a bug. Please report it."
            );
        }

        self.output_or_error = Some(output);
    }

    #[doc(hidden)]
    pub fn into_parts(
        self,
    ) -> (
        Input,
        Option<OutputOrError>,
        Option<Request>,
        Option<Response>,
    ) {
        (
            self.input,
            self.output_or_error,
            self.request,
            self.response,
        )
    }
}
