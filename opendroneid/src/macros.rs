macro_rules! impl_message {
    (
        $message_ty:ident,
        $data_ty:ty,
        $encoded_ty:ty,
        $init:path,
        $encode:path,
        $decode:path
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $message_ty {
            data: $data_ty,
        }

        impl $message_ty {
            /// Creates a new message with default values.
            /// The values are the default values defined by the underlying C library,
            /// which may not be valid.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl MessageInternal for $message_ty {
            type Data = $data_ty;
            type Encoded = $encoded_ty;

            fn init_data(data: *mut Self::Data) {
                unsafe { $init(data) }
            }

            fn encode_message(
                out_encoded: *mut Self::Encoded,
                in_data: *const Self::Data,
            ) -> Result<(), EncodeError> {
                if unsafe { $encode(out_encoded, in_data) } as u32 != sys::ODID_SUCCESS {
                    return Err(EncodeError::Unknown(stringify!($message_ty).to_string()));
                }
                Ok(())
            }

            fn decode_message(
                out_data: *mut Self::Data,
                in_encoded: *const Self::Encoded,
            ) -> Result<(), DecodeError> {
                if unsafe { $decode(out_data, in_encoded) } as u32 != sys::ODID_SUCCESS {
                    return Err(DecodeError::Unknown(stringify!($message_ty).to_string()));
                }
                Ok(())
            }
        }

        impl Message for $message_ty {
            type Data = $data_ty;
            type Encoded = $encoded_ty;

            fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
                let encoded_len = self.encoded_len();
                if buf.remaining_mut() < encoded_len {
                    return Err(EncodeError::BufferTooSmall {
                        message: stringify!($message_ty).to_string(),
                        remaining: buf.remaining_mut(),
                        required: encoded_len,
                    });
                }

                let mut encoded = std::mem::MaybeUninit::<Self::Encoded>::uninit();
                <Self as MessageInternal>::encode_message(
                    encoded.as_mut_ptr(),
                    &self.data as *const Self::Data,
                )?;

                let bytes = unsafe {
                    std::slice::from_raw_parts(encoded.as_ptr().cast::<u8>(), encoded_len)
                };
                buf.put_slice(bytes);
                Ok(())
            }

            fn decode(buf: impl Buf) -> Result<Self, DecodeError> {
                let mut buf = buf;
                let encoded_len = std::mem::size_of::<Self::Encoded>();
                if buf.remaining() < encoded_len {
                    return Err(DecodeError::BufferTooSmall {
                        message: stringify!($message_ty).to_string(),
                        remaining: buf.remaining(),
                        expected: encoded_len,
                    });
                }

                let mut encoded = std::mem::MaybeUninit::<Self::Encoded>::uninit();
                let encoded_bytes = unsafe {
                    std::slice::from_raw_parts_mut(encoded.as_mut_ptr().cast::<u8>(), encoded_len)
                };
                buf.copy_to_slice(encoded_bytes);

                let mut message = Self::new();
                <Self as MessageInternal>::decode_message(
                    &mut message.data as *mut Self::Data,
                    encoded.as_ptr(),
                )?;
                Ok(message)
            }
        }

        impl Default for $message_ty {
            /// Creates a new message with default values.
            fn default() -> Self {
                let mut data = ::std::mem::MaybeUninit::<$data_ty>::uninit();
                <Self as MessageInternal>::init_data(data.as_mut_ptr());
                let data = unsafe { data.assume_init() };
                Self { data }
            }
        }
    };
}

pub(crate) use impl_message;
