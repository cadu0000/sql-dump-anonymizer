# 🦀 Blindfold: High-Throughput SQL Stream Sanitizer

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-Work_in_Progress-orange.svg)]()
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg)](#licença)

> **Aviso:** Este projeto está atualmente em fase de ideação e desenvolvimento ativo como parte de um Trabalho de Conclusão de Curso (TCC) em Sistemas de Informação. As funcionalidades descritas abaixo representam a arquitetura planejada e o roadmap da ferramenta.

**Blindfold** (`sql-dump-anonymizer`) é uma proposta de ferramenta de linha de comando (CLI) escrita em Rust. Seu objetivo é atuar como um processador de fluxo (*stream processor*) para anonimizar, pseudonimizar e aplicar Privacidade Diferencial em dumps de banco de dados SQL massivos (Terabytes) "em voo", sem a necessidade de conexões ativas com o banco ou alto consumo de memória RAM (*Zero-Memory Overhead*).

---

## 💡 O Problema e a Solução

Equipes de engenharia precisam de dados realistas para testar softwares, mas leis de privacidade (como a LGPD) proíbem o uso de dados de produção. Soluções atuais frequentemente falham por exigirem conexão de rede com o banco (inviável em ambientes *air-gapped*) ou por tentarem carregar o banco inteiro na memória, causando gargalos de performance.

O **Blindfold** resolve isso atuando nativamente com *Unix Pipes*. Ele lê o texto do dump SQL linha por linha, identifica dados sensíveis através de uma Máquina de Estados léxica, aplica algoritmos de criptografia e escreve o resultado instantaneamente na saída.



---

## 🛠️ Como vai funcionar (Arquitetura Planejada)

A ferramenta será guiada por uma **Estratégia Híbrida de Mascaramento**, dividindo o problema em três frentes:

1. **Integridade Estrutural (HMAC-SHA256):** Chaves Primárias (IDs) e Estrangeiras (FKs) sofrerão pseudonimização determinística. O ID `5` sempre virará `892`, preservando os `JOINs` sem a necessidade de manter tabelas "De-Para" na memória.
2. **Anonimização de PII (Faker):** Nomes, e-mails e CPFs serão substituídos por dados falsos e realistas.
3. **Privacidade Diferencial Local (LDP):** Para dados numéricos (ex: Salários, Idades), utilizaremos o **Mecanismo de Laplace** para injetar ruído matemático. Isso protege o indivíduo, mas mantém as médias e propriedades estatísticas intactas para as equipes de Ciência de Dados.

---

## ⚙️ Configuração como Código (Configuration as Code)

A execução do *Blindfold* dependerá de dois componentes fundamentais de configuração, separando regras de negócio de segredos de infraestrutura:

### 1. O Arquivo de Regras (`rules.toml`)
Este arquivo é o mapa da ferramenta. Ele **deve ser versionado no Git** para garantir que toda a equipe de desenvolvimento tenha a mesma estrutura de testes. Tabelas não declaradas aqui sofrerão *Bypass* (passarão direto para a saída).

```toml
# rules.toml (Example)

[tables.users]
columns = [
    # Deterministic Masking (Keeps JOINs working)
    { name = "id", strategy = "hmac" },
    
    # Random Anonymization (PII)
    { name = "name", strategy = "faker_name" },
    
    # Fixed Value (Allows dev team to login with a known test password)
    { name = "password_hash", strategy = "fixed", value = "$2a$12$R9h/cIPz0gi..." },
    
    # Local Differential Privacy (Laplace Mechanism for numerics)
    { name = "salary", strategy = "dp_laplace", epsilon = 0.5, sensitivity = 15000.0 }
]

```

### 2. A Chave Secreta Criptográfica (`.env`)

A chave usada para gerar os hashes HMAC. Por motivos de segurança, este valor **NUNCA deve ser commitado no repositório** (adicione o `.env` ao `.gitignore`). Ele é lido em tempo de execução via variável de ambiente.

```bash
# arquivo: .env
BLINDFOLD_SECRET="chave_super_secreta_de_producao"

```

---

## 🚀 Uso Planejado (Exemplos)

A interface de linha de comando será construída usando `clap` e suportará tanto arquivos estáticos quanto *streams* nativos do sistema operacional:

**Abordagem 1: Pipeline Unix (Zero uso de disco extra)**

```bash
zcat production_db.sql.gz | blindfold -c rules.toml | gzip > dev_db_anon.sql.gz

```

**Abordagem 2: Processamento de Arquivos**

```bash
blindfold --config rules.toml --input production_db.sql --output dev_db.sql

```

## 📄 Licença

Planejado para ser distribuído sob licença dupla **MIT** ou **Apache-2.0**.
