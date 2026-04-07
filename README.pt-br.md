# 3BEP Open Science Archive

*Leia em [Inglês (English)](README.md).*

Bem-vindo ao **3BEP Labs Archive**, o cartório da verdade física determinística.

## O Manifesto do Determinismo 3BEP

A ciência computacional moderna vive sob uma ilusão. Há décadas, a academia orbita um abismo matemático chamado **IEEE 754** (Ponto Flutuante). De simulações astrofísicas de galáxias à dinâmica interatômica para dobramento de peptídeos, pesquisadores ao redor do mundo publicam "avanços" baseados em uma infraestrutura que sofre de uma inevitável "evaporação numérica". A ordem das operações afeta o vetor resultante. Células e átomos sofrem alucinações termodinâmicas no longo prazo. O resultado? Um cenário de pesquisa onde "reprodutibilidade" é aspiracional e não garantida, e onde plataforma, compilador e até flags de otimização alteram silenciosamente a trajetória das simulações. Isso não é computação determinística; é aproximação estocástica com maquiagem estatística.

Nós do **3BEP Labs** estabelecemos o "Santuário", um núcleo construído para abrigar apenas matemática estrita **I64F64**, onde cada fração é processada sobre as fundações inabaláveis de inteiros grandes (Determinismo Puro em Ponto Fixo de 128 Bits). Na nossa liturgia de código, a Primeira Lei da Termodinâmica não é uma tolerância métrica, **ela é coercitiva**. 

### Nossa Missão

Este repositório encerra as tolerâncias perdoáveis. Ele será o repositório jurídico-tecnológico para a nossa missão:

1. **Digital Twin of Science (Cemitério de Teorias):** Identificaremos artigos acadêmicos (arXiv, e similares) em que a fragilidade do modelo desmorona a estabilidade propagada. Reproduziremos os cálculos base através da matemática bit-a-bit do 3BEP para provar o momento e instante exatos de quando suas teses desabam no infinito;
2. **Prior Art Incontestavel:** O codigo base de matematica do 'Santuario' sera auditavel publicamente em Rust rigorosamente testado, desmentindo que dependemos de truques para estabilizar as simulacoes, com selos de integridade criptografica no roadmap.

**"A verdade física pertence a todos, mas a exploração comercial da nossa precisão exige o reconhecimento direto do Arquiteto."**

## Licença AGPL v3.0 e Soberania Tecnológica

Nosso trabalho — os Motores I64F64 nativos, o pipeline de verificacao deterministica, e a traducao do Universo via Rust puro (`#![no_std]`) — e protegido e exposto atraves da [GNU Affero General Public License v3.0 (AGPL-3.0)](LICENSE).

Isso confere o peso definitivo da estratégia de soberania:
- **Para Auditar a Ciência (Acadêmicos):** A transparência é absoluta. Sintam-se convidados a clonar, auditar e testar nossa infra-estrutura como prova de autoridade.
- **Para Exibir Rentabilidade (Cloud/SaaS/Corporações):** A 'Brecha de Serviços Cloud' foi lacrada. Aqueles que usarem de nosso motor determinístico via rede ou serviços devem abrir 100% de seu ecossistema. Qualquer comercialização ou adaptação "software-as-a-service" que procure escapar destas diretrizes requer estritamente o recolhimento de uma **Licença Comercial (Dual-Licensing)** via contato conosco.

## Como Auditar (Guia Rápido)

```bash
# 1. Clonar o repositório
git clone https://github.com/3beplabs/3bep-open-science-archive.git
cd 3bep-open-science-archive/core_engine

# 2. Executar a suíte completa de testes (30 testes, Protocolo de Tolerância Zero)
cargo test

# 3. Executar a demonstração de órbita estável (100k ticks)
cargo run --example example_1_stable_orbit --release

# 4. Executar o teste de estresse extremo (50M ticks, CPU burn-in)
cargo run --example extreme_stress_test --release
```

Todos os testes validam determinismo bit-a-bit, conservacao de energia, imunidade a singularidades, precisao analitica Kepleriana, integracao simpletica, prova de divergencia IEEE 754, escalabilidade N-body, conservacao de momento, e reversibilidade temporal. Veja [TESTS.md](TESTS.md) para o registro detalhado de execucao.

## Validador CLI (Auditoria sem Friccao)

A ferramenta `cli_3bep` permite que pesquisadores validem fisica **sem escrever nenhuma linha de Rust**. Defina seu experimento em JSON e execute:

### 1. Crie seu arquivo de experimento (`meu_experimento.json`):

```json
{
  "experiment_name": "Orbita_Kepler_Circular",
  "bodies": [
    { "mass": 1000.0, "pos": [0, 0, 0], "vel": [0, 0, 0] },
    { "mass": 1.0, "pos": [10, 0, 0], "vel": [0, 10, 0] }
  ],
  "integrator": "leapfrog",
  "dt": "0.01",
  "steps": 6280
}
```

### 2. Execute a validacao:

```bash
# Simulacao I64F64 basica com relatorio de energia/momento
cd cli_3bep
cargo run --release -- validate meu_experimento.json

# Exportar trajetoria completa em CSV (posicao, velocidade, energia a cada N passos)
cargo run --release -- validate meu_experimento.json --trajectory

# Comparar I64F64 vs IEEE 754 (f64) — veja a divergencia exata
cargo run --release -- validate meu_experimento.json --compare-with-f64

# Exportar estado final como JSON (inclui hash deterministico)
cargo run --release -- validate meu_experimento.json --export json
```

### 3. A saida inclui:
- **Conservacao de energia** (inicial vs final, drift)
- **Conservacao de momento** (dPx, dPy com 14 casas decimais)
- **Estado final** de todos os corpos (posicao + velocidade)
- **Hash deterministico** (fingerprint FNV-1a para reprodutibilidade)
- **Trajetoria CSV completa** (ao usar `--trajectory`): step, time, body, pos_xyz, vel_xyz, energia, momento — pronto para matplotlib/gnuplot
- **Comparacao IEEE 754** (ao usar `--compare-with-f64`)

### Referencia dos Campos JSON:
| Campo | Tipo | Descricao |
|---|---|---|
| `experiment_name` | string | Nome para identificacao |
| `bodies` | array | Lista de corpos com mass, pos[x,y,z], vel[x,y,z] |
| `integrator` | string | `"rk4"` ou `"leapfrog"` |
| `dt` | string | Passo temporal (string para preservar precisao I64F64) |
| `steps` | inteiro | Numero de passos de integracao |
| `export_interval` | inteiro | (opcional) Salvar trajetoria a cada N passos. Padrao: 1 |

Veja `cli_3bep/examples/kepler_orbit.json` para um exemplo funcional.

## Arquitetura do Motor

O Santuário oferece **dois integradores** para diferentes cenários científicos:

| Integrador | Ordem | Melhor Para | Comportamento Energético |
|---|---|---|---|
| **RK4** (`rk4.rs`) | O(h⁴) | Simulações curtas de alta precisão | Drift secular linear |
| **Leapfrog** (`leapfrog.rs`) | O(h²) | Estabilidade de longo prazo, sistemas caóticos | Oscilação limitada (simplético) |

Ambos os integradores estão disponíveis para o sistema fixo de 3 corpos e para o sistema genérico de N corpos (`nbody.rs`).

## Estrutura do Repositório

* `core_engine/src/physics/` — O Kernel Santuário:
  - `vector3.rs` — Aritmética vetorial I64F64 com raiz quadrada Newton-Raphson
  - `constants.rs` — Parâmetros físicos (G, DT, SOFTENING)
  - `rk4.rs` — Integrador Runge-Kutta clássico de 4ª ordem (3 corpos)
  - `leapfrog.rs` — Integrador simplético Velocity Verlet (3 corpos)
  - `nbody.rs` — Sistema genérico de N corpos com RK4 e Leapfrog
* `core_engine/tests/` — Suite de Tolerancia Zero (13 modulos, 30 testes). Veja [TESTS.md](TESTS.md).
* `core_engine/examples/` — Demonstracoes executaveis para verificacao independente.
* `cli_3bep/` — Validador JSON sem friccao. Veja [Validador CLI](#validador-cli-auditoria-sem-friccao) acima.
* `preprint_archaeology/` — Evidencias, divergencias mapeadas, e selos de integridade criptografica *(em breve)*.

## Alegações Científicas Chave (Provadas por Testes)

1. **Determinismo Bit-a-Bit:** Entradas idênticas sempre produzem saídas idênticas, independentemente da ordem de execução ou plataforma. *(Testes: chaos_3body, leapfrog_conservation, nbody_scalability)*
2. **Precisão Kepleriana Analítica:** Erro de retorno orbital < 2% por órbita, velocidade conservada a 0.015%, razão de drift linear em exatamente 5.0x. *(Teste: kepler_validation)*
3. **Divergência IEEE 754:** f64 e I64F64 produzem trajetórias mensuravelmente diferentes a partir do passo 507 para as mesmas condições iniciais. *(Teste: f64_divergence)*
4. **Conservação Simplética de Energia:** Leapfrog conserva energia 4x melhor que RK4 em regimes caóticos, com drift máximo de 0.000003 em 200 órbitas Keplerianas. *(Teste: leapfrog_conservation)*
5. **Escalabilidade N-Body:** Sistemas de 5 e 10 corpos mantêm determinismo total em todas as coordenadas. *(Teste: nbody_scalability)*
6. **Imunidade a Singularidades:** Sem NaN, sem overflow, sem pânico sob forças gravitacionais extremas (r → 0). *(Teste: singularity_stress)*
7. **Conservação de Momento (Newton III):** Momento linear total conservado a **14 casas decimais** no Kepler e **13 casas decimais** no caos. *(Teste: momentum_conservation)*
8. **Momento Angular (2ª Lei de Kepler):** Conservado a 10 dígitos significativos em 100 órbitas (erro relativo 8.8×10⁻¹⁰). *(Teste: angular_momentum)*
9. **Reversibilidade Temporal:** Leapfrog retorna ao estado inicial com erro de 5.4×10⁻¹⁷ após 1.000 passos para frente + 1.000 para trás — **43 milhões de vezes mais reversível** que RK4. *(Teste: time_reversibility)*
10. **Órbita Elíptica (Kepler I + Vis-Viva):** Distância de afélio coincide com a previsão analítica em 0.5% para e=0.5. Equação vis-viva v² = GM(2/r − 1/a) válida a 1.8% em toda a órbita. *(Teste: elliptical_orbit)*
11. **Verificação de Ordem de Convergência:** Erro de energia RK4 converge na razão **32.0** (confirmando O(h⁵)), erro de posição Leapfrog converge na razão **4.0** (confirmando O(h²)). Ambos coincidem com previsões teóricas a 3+ dígitos significativos. *(Teste: convergence_order)*
12. **Determinismo Cross-Platform (Provado Empiricamente):** Todos os 30 testes produzem resultados **bit-a-bit idênticos** em 3 máquinas: AMD Ryzen (Windows 11), AMD EPYC (Ubuntu 24.04), e Intel Core i5-6200U (Windows 10). Cada dígito, cada bit, cada trajetória — idêntica. *(Veja: TESTS.md, seção Cross-Platform)*

## Referencias

Todas as constantes fisicas, algoritmos e alegacoes teoricas sao respaldados por fontes academicas primarias. Veja [REFERENCES.md](REFERENCES.md) para a lista completa de citacoes incluindo valores NIST CODATA, papers originais de Runge (1895), Verlet (1967), Noether (1918), e o padrao IEEE 754-2019.

## Contribuicao & Ciencia Aberta

Este projeto existe para servir a ciencia, nao para controla-la. O motor Santuario pertence a comunidade sob a licenca AGPL-3.0, e convidamos ativamente a participacao:

**Proponha um Teste.** Se voce acredita que existe um cenario fisico que desafia nosso motor deterministico, abra uma Issue com suas condicoes iniciais em formato JSON. Nos executaremos, publicaremos os resultados de forma transparente, e adicionaremos a suite de testes se revelar algo significativo. Nao temos medo de estar errados — temos medo de nao saber.

**Envie Resultados Cross-Platform.** Rode `cargo test` na sua maquina e compartilhe a saida. Cada nova arquitetura que produz resultados bit-identicos fortalece a prova. Cada uma que nao produz revela algo que precisamos corrigir. Ambos os resultados sao valiosos.

**Desafie Nossas Alegacoes.** Cada uma das 12 alegacoes cientificas listadas acima esta ligada a um teste especifico e reproduzivel. Se voce encontrar uma falha na nossa metodologia, um bug na nossa matematica, ou uma suposicao que nao justificamos — nos diga. O proposito inteiro de publicar o motor e convidar escrutinio.

**Use a CLI para Seus Papers.** O validador `cli_3bep` foi construido para que fisicos possam verificar suas proprias simulacoes sem aprender Rust. Se voce publicar um paper usando dados de preprints do arXiv, pode rodar suas condicoes iniciais pelo nosso motor e incluir o hash deterministico nos seus materiais suplementares como certificado de reprodutibilidade.

**O Que Isto Nao E.** Isto nao e um produto comercial disfarcado de open source. Isto nao e uma ferramenta projetada para envergonhar pesquisadores. O "Cemiterio de Teorias" existe porque a reprodutibilidade numerica e uma crise na fisica computacional — nao porque nos consideramos superiores a comunidade academica. Nos construimos o chao; queremos que todos pisem nele.

> *"O objetivo nao e provar que estamos certos. O objetivo e tornar impossivel que qualquer um — incluindo nos mesmos — esteja errado sem saber."*

---
**3BEP Labs** | A Infraestrutura da Verdade Fisica.
