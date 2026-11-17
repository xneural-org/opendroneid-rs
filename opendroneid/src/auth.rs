//! Authentication message and its pages.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::{
    constants::{AUTH_FIRST_PAGE_DATA_SIZE, AUTH_MAX_PAGES, AUTH_NEXT_PAGE_DATA_SIZE},
    error::Error,
    macros,
    message::{Message, MessageInternal},
};
#[cfg(feature = "chrono")]
use crate::{from_odid_epoch, to_odid_epoch};

/// Type of authentication used in the [`Auth`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum AuthenticationType {
    #[default]
    None = sys::ODID_authtype_ODID_AUTH_NONE,
    UasIdSignature = sys::ODID_authtype_ODID_AUTH_UAS_ID_SIGNATURE,
    OperatorIdSignature = sys::ODID_authtype_ODID_AUTH_OPERATOR_ID_SIGNATURE,
    MessageSetSignature = sys::ODID_authtype_ODID_AUTH_MESSAGE_SET_SIGNATURE,
    NetworkRemoteId = sys::ODID_authtype_ODID_AUTH_NETWORK_REMOTE_ID,
    SpecificAuthentication = sys::ODID_authtype_ODID_AUTH_SPECIFIC_AUTHENTICATION,
}

/// Page of the authentication data in an [`Auth`] message. The variant is the page number.
///
/// The first page holds the fields that describe the whole authentication data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthPage {
    /// Page 0.
    First {
        /// Index of the last page, below [`AUTH_MAX_PAGES`].
        last_page_index: u8,
        /// Length in bytes of the authentication data across all pages.
        length: u8,
        /// Seconds since 2019-01-01 00:00:00 UTC.
        timestamp: u32,
        /// First bytes of the authentication data.
        data: [u8; AUTH_FIRST_PAGE_DATA_SIZE],
    },
    /// Pages 1 to `AUTH_MAX_PAGES - 1`.
    Next {
        /// Page number.
        index: u8,
        /// Following bytes of the authentication data.
        data: [u8; AUTH_NEXT_PAGE_DATA_SIZE],
    },
}

impl AuthPage {
    /// Returns the page number.
    pub fn index(&self) -> u8 {
        match self {
            AuthPage::First { .. } => 0,
            AuthPage::Next { index, .. } => *index,
        }
    }

    /// Returns the bytes of the authentication data on this page.
    pub fn data(&self) -> &[u8] {
        match self {
            AuthPage::First { data, .. } => data,
            AuthPage::Next { data, .. } => data,
        }
    }

    /// Returns how many bytes of authentication data fit in pages 0 to `last_page_index`.
    fn capacity(last_page_index: u8) -> usize {
        AUTH_FIRST_PAGE_DATA_SIZE + last_page_index as usize * AUTH_NEXT_PAGE_DATA_SIZE
    }
}

macros::impl_message!(
    /// The Authentication Message defines a field that provides a means for authenticating the
    /// identity of the UA sending the message.
    ///
    /// The authentication data is split across up to [`AUTH_MAX_PAGES`] messages, one per
    /// [`AuthPage`].
    ///
    /// ```rust
    /// use opendroneid::{Auth, AuthPage, AUTH_FIRST_PAGE_DATA_SIZE};
    ///
    /// // Create the first page of an Auth message.
    /// let auth = Auth::default()
    ///     // Set fields of the Auth message using the builder pattern.
    ///     .with_auth_type(opendroneid::AuthenticationType::UasIdSignature)
    ///     .with_page(AuthPage::First {
    ///         last_page_index: 0,
    ///         length: 4,
    ///         timestamp: 12345,
    ///         data: [0xde, 0xad, 0xbe, 0xef, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     })
    ///     .expect("Invalid Auth page");
    ///
    /// assert_eq!(auth.page().data().len(), AUTH_FIRST_PAGE_DATA_SIZE);
    /// ```
    Auth,
    sys::ODID_Auth_data,
    sys::ODID_Auth_encoded,
    sys::odid_initAuthData,
    sys::encodeAuthMessage,
    sys::decodeAuthMessage
);

impl Auth {
    /// Returns the authentication type of the message, or an [`Error`] if the value is invalid.
    pub fn auth_type(&self) -> Result<AuthenticationType, Error> {
        AuthenticationType::from_u32(self.data.AuthType).ok_or(Error::EnumMappingError {
            field: "AuthType",
            value: self.data.AuthType,
        })
    }

    /// Sets the authentication type of the message.
    pub fn with_auth_type(mut self, auth_type: AuthenticationType) -> Self {
        self.data.AuthType = auth_type as u32;
        self
    }

    /// Returns the page of the authentication data in the message.
    ///
    /// The variant of the [`AuthPage`] is the page number of the message.
    pub fn page(&self) -> AuthPage {
        if self.data.DataPage == 0 {
            let mut data = [0u8; AUTH_FIRST_PAGE_DATA_SIZE];
            data.copy_from_slice(&self.data.AuthData[..AUTH_FIRST_PAGE_DATA_SIZE]);
            AuthPage::First {
                last_page_index: self.data.LastPageIndex,
                length: self.data.Length,
                timestamp: self.data.Timestamp,
                data,
            }
        } else {
            let mut data = [0u8; AUTH_NEXT_PAGE_DATA_SIZE];
            data.copy_from_slice(&self.data.AuthData[..AUTH_NEXT_PAGE_DATA_SIZE]);
            AuthPage::Next {
                index: self.data.DataPage,
                data,
            }
        }
    }

    /// Sets the page of the authentication data and its page number.
    ///
    /// Returns an error if a page index is out of range, or if the `length` of the first page
    /// exceeds the data that fits in the pages up to its `last_page_index`.
    pub fn with_page(mut self, page: AuthPage) -> Result<Self, Error> {
        self.data.AuthData.fill(0);
        match page {
            AuthPage::First {
                last_page_index,
                length,
                timestamp,
                data,
            } => {
                if last_page_index as usize >= AUTH_MAX_PAGES {
                    return Err(Error::InvalidValue {
                        field: "last_page_index",
                        value: last_page_index.to_string(),
                    });
                }
                if length as usize > AuthPage::capacity(last_page_index) {
                    return Err(Error::InvalidValue {
                        field: "length",
                        value: length.to_string(),
                    });
                }
                self.data.DataPage = 0;
                self.data.LastPageIndex = last_page_index;
                self.data.Length = length;
                self.data.Timestamp = timestamp;
                self.data.AuthData[..AUTH_FIRST_PAGE_DATA_SIZE].copy_from_slice(&data);
            }
            AuthPage::Next { index, data } => {
                if index == 0 || index as usize >= AUTH_MAX_PAGES {
                    return Err(Error::InvalidValue {
                        field: "index",
                        value: index.to_string(),
                    });
                }
                // Only the first page carries these fields.
                self.data.DataPage = index;
                self.data.LastPageIndex = 0;
                self.data.Length = 0;
                self.data.Timestamp = 0;
                self.data.AuthData[..AUTH_NEXT_PAGE_DATA_SIZE].copy_from_slice(&data);
            }
        }
        Ok(self)
    }

    /// Returns the timestamp of the first page as a `chrono::DateTime<Utc>`, or `None` for a
    /// later page.
    #[cfg(feature = "chrono")]
    pub fn chrono_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self.page() {
            AuthPage::First { timestamp, .. } => Some(from_odid_epoch(timestamp)),
            AuthPage::Next { .. } => None,
        }
    }

    /// Sets the timestamp of the first page to `time`, rounded down to whole seconds.
    ///
    /// Returns an error for a later page, which has no timestamp, and for a time before 2019 or
    /// after the range of the timestamp.
    #[cfg(feature = "chrono")]
    pub fn with_chrono_timestamp(
        mut self,
        time: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, Error> {
        if self.data.DataPage != 0 {
            return Err(Error::InvalidValue {
                field: "timestamp",
                value: format!("page {} has no timestamp", self.data.DataPage),
            });
        }
        self.data.Timestamp = to_odid_epoch(time)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;

    fn first_page(last_page_index: u8, length: u8) -> AuthPage {
        let mut data = [0u8; AUTH_FIRST_PAGE_DATA_SIZE];
        data[0] = 0x00;
        data[1] = 0xff;
        data[AUTH_FIRST_PAGE_DATA_SIZE - 1] = 0x80;
        AuthPage::First {
            last_page_index,
            length,
            timestamp: 12345,
            data,
        }
    }

    #[test]
    fn auth_pages_round_trip() {
        let mut data = [0xa5u8; AUTH_NEXT_PAGE_DATA_SIZE];
        data[0] = 0x00;
        for page in [
            first_page(2, 63),
            AuthPage::Next { index: 1, data },
            AuthPage::Next {
                index: AUTH_MAX_PAGES as u8 - 1,
                data,
            },
        ] {
            let auth = Auth::default()
                .with_auth_type(AuthenticationType::MessageSetSignature)
                .with_page(page)
                .unwrap();

            let decoded = round_trip(&auth);

            assert_eq!(
                decoded.auth_type().unwrap(),
                AuthenticationType::MessageSetSignature
            );
            assert_eq!(decoded.page(), page);
            assert_eq!(decoded.page().index(), page.index());
        }
    }

    #[test]
    fn auth_length_must_fit_in_the_pages() {
        let capacity = (AUTH_FIRST_PAGE_DATA_SIZE + AUTH_NEXT_PAGE_DATA_SIZE) as u8;

        assert!(Auth::default().with_page(first_page(1, capacity)).is_ok());
        assert!(matches!(
            Auth::default().with_page(first_page(1, capacity + 1)),
            Err(Error::InvalidValue {
                field: "length",
                ..
            })
        ));
    }

    #[test]
    fn auth_page_indices_are_checked() {
        let data = [0u8; AUTH_NEXT_PAGE_DATA_SIZE];
        for index in [0, AUTH_MAX_PAGES as u8] {
            assert!(matches!(
                Auth::default().with_page(AuthPage::Next { index, data }),
                Err(Error::InvalidValue { field: "index", .. })
            ));
        }
        assert!(matches!(
            Auth::default().with_page(first_page(AUTH_MAX_PAGES as u8, 0)),
            Err(Error::InvalidValue {
                field: "last_page_index",
                ..
            })
        ));
    }

    #[test]
    fn later_auth_page_clears_first_page_fields() {
        let data = [0x11u8; AUTH_NEXT_PAGE_DATA_SIZE];
        let auth = Auth::default()
            .with_page(first_page(3, 40))
            .unwrap()
            .with_page(AuthPage::Next { index: 2, data })
            .unwrap();

        let expected = Auth::default()
            .with_page(AuthPage::Next { index: 2, data })
            .unwrap();
        assert_eq!(auth, expected);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn auth_chrono_timestamp_is_only_on_the_first_page() {
        let first = Auth::default().with_page(first_page(0, 1)).unwrap();
        let next = Auth::default()
            .with_page(AuthPage::Next {
                index: 1,
                data: [0; AUTH_NEXT_PAGE_DATA_SIZE],
            })
            .unwrap();

        assert_eq!(first.chrono_timestamp(), Some(from_odid_epoch(12345)));
        assert_eq!(next.chrono_timestamp(), None);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn auth_chrono_timestamp_is_set_on_the_first_page() {
        let time = utc("2026-11-10T15:42:17Z");
        let auth = Auth::default()
            .with_page(first_page(0, 1))
            .and_then(|auth| auth.with_chrono_timestamp(time))
            .unwrap();

        assert_eq!(round_trip(&auth).chrono_timestamp(), Some(time));
        assert!(matches!(auth.page(), AuthPage::First { length: 1, .. }));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn auth_chrono_timestamp_is_rejected_on_a_later_page_and_before_2019() {
        let next = Auth::default()
            .with_page(AuthPage::Next {
                index: 1,
                data: [0; AUTH_NEXT_PAGE_DATA_SIZE],
            })
            .unwrap();
        let first = Auth::default().with_page(first_page(0, 1)).unwrap();

        for result in [
            next.with_chrono_timestamp(utc("2026-11-10T15:42:17Z")),
            first.with_chrono_timestamp(utc("2018-12-31T23:59:59Z")),
        ] {
            assert!(matches!(
                result,
                Err(Error::InvalidValue {
                    field: "timestamp",
                    ..
                })
            ));
        }
    }
}
