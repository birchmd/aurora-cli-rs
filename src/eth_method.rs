use aurora_engine_transactions::EthTransactionKind;
use aurora_engine_types::{types::Address, H256};

pub enum EthMethod {
    GetChainId,
    GetTransactionCount(Address),
    GetTransactionReceipt(H256),
    DebugTraceTransaction(H256),
    SendRawTransaction(Box<EthTransactionKind>),
    Call(EthCall),
}

impl EthMethod {
    pub fn name(&self) -> &'static str {
        match &self {
            Self::GetChainId => "net_version",
            Self::GetTransactionCount(_) => "eth_getTransactionCount",
            Self::GetTransactionReceipt(_) => "eth_getTransactionReceipt",
            Self::DebugTraceTransaction(_) => "debug_traceTransaction",
            Self::SendRawTransaction(_) => "eth_sendRawTransaction",
            Self::Call(_) => "eth_call",
        }
    }

    pub fn create_params(&self) -> Vec<serde_json::Value> {
        match &self {
            Self::GetChainId => Vec::new(),
            Self::GetTransactionCount(address) => {
                vec![serde_json::Value::String(format!("0x{}", address.encode()))]
            }
            Self::GetTransactionReceipt(tx_hash) => {
                vec![serde_json::Value::String(format!("0x{}", hex::encode(tx_hash)))]
            }
            Self::DebugTraceTransaction(tx_hash) => {
                vec![serde_json::Value::String(format!("0x{}", hex::encode(tx_hash)))]
            }
            Self::SendRawTransaction(tx) => {
                let tx_bytes: Vec<u8> = tx.as_ref().into();
                vec![serde_json::Value::String(format!("0x{}", hex::encode(tx_bytes)))]
            }
            Self::Call(args) => vec![args.to_json()],
        }
    }
}

#[derive(Debug)]
pub struct EthCall {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub data: Option<Vec<u8>>,
}

impl EthCall {
    pub fn to_json(&self) -> serde_json::Value {
        let mut obj = serde_json::Map::new();

        if let Some(addr) = self.from.as_ref() {
            obj.insert("from".into(), serde_json::Value::String(format!("0x{}", addr.encode())));
        }

        if let Some(addr) = self.to.as_ref() {
            obj.insert("to".into(), serde_json::Value::String(format!("0x{}", addr.encode())));
        }

        if let Some(data) = self.data.as_ref() {
            obj.insert("data".into(), serde_json::Value::String(format!("0x{}", hex::encode(data))));
        }

        serde_json::Value::Object(obj)
    }
}
