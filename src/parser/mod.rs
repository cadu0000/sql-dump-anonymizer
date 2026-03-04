pub mod state;

pub use state::SqlDialect;
use state::{
    InsertHeaderEvent, InsertHeaderState, NormalEvent, NormalState, State, ValueEvent, ValueState,
};

pub struct SqlParser {
    state: State,
    dialect: SqlDialect,
}

impl SqlParser {
    pub fn new(dialect: SqlDialect) -> Self {
        Self {
            state: State::Normal(NormalState::new()),
            dialect,
        }
    }

    pub fn handle_byte(&mut self, byte: u8) -> Option<Vec<u8>> {
        match &mut self.state {
            State::Normal(normal_state) => match normal_state.process_byte(byte) {
                NormalEvent::StartInsertHeader => {
                    self.state = State::InsertHeader(InsertHeaderState::new(self.dialect));
                    None
                }
                NormalEvent::Continue => None,
            },
            State::InsertHeader(header_state) => {
                match header_state.process_byte(byte) {
                    InsertHeaderEvent::HeaderComplete(format) => {
                        let v_state = ValueState::new(self.dialect, format);
                        self.state = State::ValueMode(v_state);
                        None
                    }
                    InsertHeaderEvent::Continue => None,
                }
            }
            State::ValueMode(v_state) => match v_state.process_byte(byte) {
                ValueEvent::TupleComplete(data) => Some(data),
                ValueEvent::ExitValuesMode => {
                    self.state = State::Normal(NormalState::new());
                    None
                }
                ValueEvent::Continue => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_parser_com_insert_tradicional() {
        let mut parser = SqlParser::new(SqlDialect::Mysql);
        let sql = b"INSERT INTO usuarios (id, nome) VALUES (1, 'Alice'), (2, 'Bob, o Construtor');";
        let mut tuplas_extraidas = Vec::new();

        for &byte in sql {
            if let Some(dados) = parser.handle_byte(byte) {
                let string_tupla = String::from_utf8_lossy(&dados).into_owned();
                tuplas_extraidas.push(string_tupla);
            }
        }

        assert_eq!(tuplas_extraidas.len(), 2);
        assert_eq!(tuplas_extraidas[0], "(1, 'Alice')");
        assert_eq!(tuplas_extraidas[1], "(2, 'Bob, o Construtor')");
    }

    #[test]
    fn test_parser_com_postgres_copy() {
        let mut parser = SqlParser::new(SqlDialect::Postgres);
        let sql = b"COPY public.usuarios (id, nome) FROM stdin;\n1\tAlice\n2\tBob\n\\.\n";
        let mut linhas_extraidas = Vec::new();

        for &byte in sql {
            if let Some(dados) = parser.handle_byte(byte) {
                let string_linha = String::from_utf8_lossy(&dados).into_owned();
                linhas_extraidas.push(string_linha);
            }
        }

        assert_eq!(linhas_extraidas.len(), 2);
        assert_eq!(linhas_extraidas[0], "1\tAlice\n");
        assert_eq!(linhas_extraidas[1], "2\tBob\n");
    }
}
