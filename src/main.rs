use std::thread::sleep;
use std::time::Duration;

const _VIAH_MARGEM: f64 = 15.0; // metros
const _VIAV_MARGEM: f64 = 15.0; // metros

const VIAH_LARGURA: f64 = 4.0; // metros
const VIAV_LARGURA: f64 = 4.0; // metros

const _VIAH_PERIMETRO: f64 = 150.0; // metros
const _VIAV_PERIMETRO: f64 = 150.0; // metros

const _CARRO_LARGURA: f64 = 2.0; // metros
const CARRO_COMPRIMETO: f64 = 4.0; // metros

// Velocidade  máxima de qualquer veiculo em metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
// Aceleração máxima de qualquer veiculo em metros por segundo ao quadrado
const ACELERACAO_MAXIMA: f64 = 3.0;
// Aceleração máxima de qualquer veiculo em metros por segundo ao quadrado
const ACELERACAO_MINIMA: f64 = -10.0;

// Simula 2 carros ate sairem do perimetro controlado au colidirem
// Retorna se houve colisão ou não

fn simula_carros(via_carro1: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64) -> bool {
    // Descrição carro 1
    let mut placa1: String = String::from("ABC1234"); // identificação de um carro
    let via1: char = via_carro1; // via deste carro
    let _acel_max1 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado 
    let _acel_min1 = ACELERACAO_MINIMA; // metros por segundo ao quadrado
    let vel_max1 = VELOCIDADE_MAXIMA; // metros por segundo
    let comprimento1 = CARRO_COMPRIMETO; // metros
    let mut pos_atual1: f64 = -80.0; // metros do cruzamento
    let mut vel_atual1: f64 = 0.0; // metros por segundo
    let acel_atual1: f64; // metros por segundo ao quadrado

    // Descrição carro 2
    let mut placa2: String = String::from("xyz9876"); // identificação de um carro    
    let via2: char = via_carro2; // via deste carro        
    let _acel_max2 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado                 
    let _acel_min2 = ACELERACAO_MINIMA; // metros por segundo ao quadrado                 
    let comprimento2 = CARRO_COMPRIMETO; // metros por segundo                     
    let vel_max2 = VELOCIDADE_MAXIMA; // metros                 
    let mut pos_atual2: f64 = -100.0; // metros do cruzamento            
    let mut vel_atual2: f64 = 0.0; // metros por segundo        
    let acel_atual2: f64; // metros por segundo ao quadrado

    // verifica a validade das placas
    placa1 = placa1.to_uppercase();
    placa2 = placa2.to_uppercase();

    if !valida_placa(&placa1) {
        panic!("    Placa inválida: {placa1}");
    }

    if !valida_placa(&placa2) {
        panic!("    Placa inválida: {placa2}");
    }

    acel_atual1 = acel_carro1;
    acel_atual2 = acel_carro2;

    println!("Início da simulação");
    let mut tickms: f64; // tempo que passou em cada tick, milisegundos

    loop {
        // Ao final do tick devemos atualizar o estado do carro
        sleep(Duration::from_millis(100));
        tickms = 100.0;

        // Atualiza o carro 1
        let old_position = pos_atual1;
        pos_atual1 = pos_atual1
            + vel_atual1 * (tickms / 1000.0)
            + acel_atual1 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;

        vel_atual1 = vel_atual1 + acel_atual1 * (tickms / 1000.0);

        // Restrições carro 1
        if pos_atual1 < old_position {
            // Não anda pra trás
            pos_atual1 = old_position;
        }

        if vel_atual1 < 0.0 {
            // Não anda para tras
            vel_atual1 = 0.0;
        }

        if vel_atual1 > vel_max1 {
            vel_atual1 = vel_max1; // Trava na velocidade máxima
        }

        println!(
            "Carro1 {} na posição {}{}, velocidade {}, aceleração {}",
            placa1, via1, pos_atual1, vel_atual1, acel_atual1
        );

        // Atualiza o carro 2

        let old_position = pos_atual2;

        pos_atual2 = pos_atual2
            + vel_atual2 * (tickms / 1000.0)
            + acel_atual2 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        vel_atual2 = vel_atual2 + acel_atual2 * (tickms / 1000.0);

        // Restrições carro 2
        if pos_atual2 < old_position {
            // Não anda para tras
            pos_atual2 = old_position;
        }

        if vel_atual2 < 0.0 {
            // Não anda para tras
            vel_atual2 = 0.0;
        }

        if vel_atual2 > vel_max2 {
            vel_atual2 = vel_max2; // Trava na velocidade máxima
        }

        println!(
            "Carro2 {} na posição {}{}, velocidade {}, aceleração {}",
            placa2, via2, pos_atual2, vel_atual2, acel_atual2
        );

        // Detecta colisão na via H
        if via1 == 'H' && via2 == 'H' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via H");
                return true;
            }
        }
        // Detecta colisão na via V
        if via1 == 'V' && via2 == 'V' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via V");
                return true;
            }
        }

        // Detecta colisão no cruzamento
        if via1 != via2 {
            if dentro_cruzamento(pos_atual1, comprimento1, via1)
                && dentro_cruzamento(pos_atual2, comprimento2, via2)
            {
                println!("Colisão dentro do cruzamento");
                return true;
            }
        }

        // Verifica se carro 1 saiu do sistema (falta a margem)
        if pos_atual1
            > comprimento1
                + if via1 == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                }
        {
            break;
        }

        // Verifica se carro 2 saiu do sistema (falta a margem)
        if pos_atual2
            > comprimento2
                + if via2 == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                }
        {
            break;
        }
    }

    return false;
}

// Colisão de dois carros ao longo da mesma via
fn colisao_longitudinal(posisao_frente: f64, comprimento: f64, posicao_atras: f64) -> bool {
    return posisao_frente - comprimento <= posicao_atras;
}

// Detecta carro dentro do cruzamento
fn dentro_cruzamento(posicao: f64, comprimento: f64, via: char) -> bool {
    return posicao > 0.0
        && posicao
            <= comprimento
                + if via == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                };
}

fn valida_placa(placa: &str) -> bool {
    // Só aceita caracteres ASCII
    if !placa.is_ascii() {
        println!("Placa não é ASCII");
        return false;
    }

    if placa.len() != 7 {
        println!("Placa não tem o tamanho certo");
        return false;
    }

    let inicio = &placa[0..3];
    let fim = &placa[3..];

    for x in inicio.chars() {
        if !x.is_alphabetic() {
            println!("Placa não tem letras no Início");
            return false;
        }
    }

    for x in fim.chars() {
        if !x.is_ascii_digit() {
            println!("Placa não tem digitos no final");
            return false;
        }
    }

    true
}

fn main() {
    println!("Inicio do programa");
    simula_carros('H', ACELERACAO_MAXIMA / 10.0, 'H', ACELERACAO_MAXIMA);
    println!("Fim da simulação");
}
