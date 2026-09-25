# Screenshot OCR

Aplicacao nativa para Windows que permanece em segundo plano e pode ser acionada por um atalho global.

O objetivo do projeto e permitir que o usuario selecione uma area da tela, gere um screenshot e extraia automaticamente textos, caracteres e numeros presentes na imagem.

## Principais recursos

- Atalho global configuravel na primeira inicializacao.
- Persistencia da configuracao junto ao aplicativo.
- Captura de regioes da tela.
- Extracao de texto usando o OCR nativo do Windows.
- Processamento local das imagens, sem depender de servicos online.
- Binario nativo e leve, desenvolvido em Rust.

## Tecnologia

- Rust
- `global-hotkey` para o atalho global.
- `screenshots` para captura da tela.
- `windows-rs` e `Windows.Media.Ocr` para reconhecimento de texto.

## Estado do projeto

O nucleo inicial ja possui persistencia do atalho, registro da hotkey, captura explicita de regioes e processamento OCR. A interface visual para selecionar a regiao da tela ainda faz parte da proxima etapa de desenvolvimento.