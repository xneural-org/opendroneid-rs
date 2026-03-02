#[macro_export]
macro_rules! impl_message {
    (
        $message_ty:ident,
        $data_ty:ty,
        $encoded_ty:ty,
        $init:path,
        $encode:path,
        $decode:path
    ) => {
        #[derive(Clone)]
        pub struct $message_ty {
            data: $data_ty,
        }

        impl $message_ty {
            pub fn new() -> Self {
                let mut data = std::mem::MaybeUninit::<$data_ty>::uninit();
                <Self as MessageInternal>::init_data(data.as_mut_ptr());
                Self {
                    data: unsafe { data.assume_init() },
                }
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

            #[inline]
            fn encoded_len() -> usize {
                std::mem::size_of::<$encoded_ty>()
            }

            fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
                let encoded_len = Self::encoded_len();
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

            fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
                let mut buf = Vec::with_capacity(Self::encoded_len());
                self.encode(&mut buf)?;
                Ok(buf)
            }

            fn decode(buf: impl Buf) -> Result<Self, DecodeError> {
                let mut buf = buf;
                if buf.remaining() < Self::encoded_len() {
                    return Err(DecodeError::BufferTooSmall {
                        message: stringify!($message_ty).to_string(),
                        remaining: buf.remaining(),
                        expected: Self::encoded_len(),
                    });
                }

                let mut encoded = std::mem::MaybeUninit::<Self::Encoded>::uninit();
                let encoded_bytes = unsafe {
                    std::slice::from_raw_parts_mut(
                        encoded.as_mut_ptr().cast::<u8>(),
                        Self::encoded_len(),
                    )
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
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
