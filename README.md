# Mandelbrot Viewer

Um visualizador do **Conjunto de Mandelbrot** desenvolvido em **Rust** utilizando a biblioteca **Macroquad**. O projeto gera o fractal em tempo real e permite aplicar zoom utilizando o teclado.

## Funcionalidades

- Geração do Conjunto de Mandelbrot.
- Colorização baseada no número de iterações.
- Zoom em tempo real.
- Renderização em alta qualidade após parar o zoom.
- Geração da imagem diretamente em memória utilizando `Texture2D`.

## Controles

| Tecla | Ação |
|-------|------|
| `Q` | Aproximar (Zoom In) |
| `E` | Afastar (Zoom Out) |

Durante o zoom o fractal é recalculado com menos iterações para manter a fluidez. Ao soltar a tecla, a imagem é renderizada novamente utilizando mais iterações para obter maior qualidade.

## Como funciona

Para cada pixel da tela, o programa converte sua posição para um ponto do plano complexo.

Em seguida, aplica iterativamente a equação:

```
z(n + 1) = z² + c
```

onde:

- `c` é o ponto correspondente ao pixel.
- `z` inicia em `0`.

Se o valor de `|z|` ultrapassar `2`, considera-se que o ponto escapa do conjunto.

A quantidade de iterações necessárias para escapar é utilizada para gerar a cor do pixel.

Os pontos que nunca escapam (até o limite de iterações) pertencem ao conjunto e são desenhados em preto.

## Tecnologias

- Rust
- Macroquad

## Executando

Clone o repositório:

```bash
git clone https://github.com/Bezerra00/mandelbrot-viewer.git
```

Entre na pasta:

```bash
cd mandelbrot-viewer
```

Execute:

```bash
cargo run --release
```

## Possíveis melhorias

- Movimentação pelo plano complexo.
- Zoom utilizando o mouse.
- Colorização contínua (*Smooth Coloring*).
- Paralelização utilizando Rayon.
- Exportação da imagem em PNG.
- Interface com informações do zoom.
- Alteração dinâmica do número de iterações.

## Referências

- https://en.wikipedia.org/wiki/Mandelbrot_set
- https://en.wikipedia.org/wiki/Complex_number
- https://macroquad.rs/

## Licença

Este projeto está disponível sob a licença MIT.
