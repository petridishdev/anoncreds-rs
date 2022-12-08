use crate::data_types::ConversionError;

use super::schema::SchemaId;

pub const CL_SIGNATURE_TYPE: &str = "CL";

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureType {
    CL,
}

impl SignatureType {
    pub fn from_str(value: &str) -> Result<Self, ConversionError> {
        match value {
            CL_SIGNATURE_TYPE => Ok(Self::CL),
            _ => Err(ConversionError::from_msg("Invalid signature type")),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            SignatureType::CL => CL_SIGNATURE_TYPE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionData {
    pub primary: ursa::cl::CredentialPrimaryPublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation: Option<ursa::cl::CredentialRevocationPublicKey>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum CredentialDefinition {
    #[serde(rename = "1.0")]
    CredentialDefinitionV1(CredentialDefinitionV1),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinitionV1 {
    pub schema_id: SchemaId,
    #[serde(rename = "type")]
    pub signature_type: SignatureType,
    pub tag: String,
    pub value: CredentialDefinitionData,
}

impl CredentialDefinitionV1 {
    pub fn get_public_key(&self) -> Result<ursa::cl::CredentialPublicKey, ConversionError> {
        let key = ursa::cl::CredentialPublicKey::build_from_parts(
            &self.value.primary,
            self.value.revocation.as_ref(),
        )
        .map_err(|e| e.to_string())?;
        Ok(key)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialDefinitionPrivate {
    pub value: ursa::cl::CredentialPrivateKey,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CredentialKeyCorrectnessProof {
    pub value: ursa::cl::CredentialKeyCorrectnessProof,
}

impl CredentialKeyCorrectnessProof {
    pub fn try_clone(&self) -> Result<Self, ConversionError> {
        Ok(Self {
            value: self.value.try_clone().map_err(|e| e.to_string())?,
        })
    }
}
