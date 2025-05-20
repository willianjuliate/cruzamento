# 🚦 Cruzamento Automático - Versão (Simples)

Simulador em Rust de um cruzamento urbano com duas vias perpendiculares (horizontal e vertical), projetado com foco didático e modularidade. O objetivo é modelar o comportamento de veículos trafegando de forma coordenada, sem o uso de semáforos, preparando terreno para implementações com controle automatizado.

---

## 🎯 Objetivos do Projeto

* Simular o fluxo de veículos em cruzamentos urbanos simples.
* Utilizar **Rust** como base para estruturas seguras e performáticas.
* Servir como base para futuras simulações com controle inteligente (ex: CVC).
* Explorar enums, structs, ownership e lógica de tempo/eventos.

---

## ⚙️ Funcionalidades

* Representação do cruzamento em estrutura de dados.
* Modelagem de veículos com direção e velocidade.
* Simulação passo a passo de movimentação nas vias.
* Priorização simples de fluxos conflitantes (em andamento).

---

## 🧱 Estrutura do Projeto

```
📦 cruzamento-automatico/
 ┣ 📜 Cargo.toml
 ┣ 📜 README.md
 ┣ 📁 src/
 ┃ ┣ 📜 main.rs         // ponto de entrada
 ┃ ┣ 📜 model.rs        // structs: Veiculo, Cruzamento, Via
 ┃ ┣ 📜 sim.rs          // lógica de simulação
 ┗ ┗ 📜 utils.rs        // funções auxiliares (ex: tempo, randomização)
 
```

---

## 🚏 Esquema do Cruzamento

```
                       largura V
                margem V|    |
                        |    |
                        |    |margem H
------------------------+----+--------
    ViaH   > > >        |    |        largura H
------------------------+----+--------
  perímetro H           |    |
                        |    |
                        |    |
                        | ^  | perímetro V
                        | ^  |
                        | ^  |
                        |ViaV|
                        |    |
```

---

## 🧪 Como Rodar

### Requisitos

* [Rust toolchain](https://www.rust-lang.org/tools/install)

### Executando

```bash
git clone https://github.com/seu-usuario/cruzamento-automatico.git
cd cruzamento-automatico
cargo run
```

### Testes

```bash
cargo test
```

---

## 🔭 Futuras Expansões

* 🚗 Simulação de múltiplos veículos com rotas distintas
* ⏱️ Temporização realista (tempo de entrada, delay)
* 🤖 Controle Central (CVC) com algoritmos de otimização
* 🖼️ Visualização em terminal (ASCII) ou gráfica (ex: Bevy, egui)

---

## 📚 Referência

Este projeto se inspira no estudo de **Oliveira, Farges, Moreira e Kraus (2002)**, publicado pela **UFSC**, que propõe o uso de um **Controlador Veicular Centralizado (CVC)** para coordenar a travessia de veículos em cruzamentos urbanos, substituindo semáforos por algoritmos de otimização baseados em branch-and-bound para minimizar atrasos e evitar colisões.

> Oliveira, R. S., Farges, J.-L., Moreira, G. O., Kraus Jr., W. (2002). *Controle ótimo de um cruzamento automatizado de tráfego urbano*. UFSC/ONERA.

---

## 👨‍💻 Autor

**Willian (Will)**
Analista de sistemas, programador Rust e amante de simulações realistas que rodem limpas e rápidas como um carro com motor boxer.

---

## 📜 Licença

Este projeto está sob a licença MIT. Sinta-se livre para usar, modificar e compartilhar. Só não tente colocar semáforo onde não precisa 😁

---

> *"Programar é transformar movimento em lógica."*