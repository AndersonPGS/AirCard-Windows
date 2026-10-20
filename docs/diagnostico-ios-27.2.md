# AirCard no iOS 27.2 RC: diagnóstico

Análise de 20/10/2026. Base: `dhava-gautama/AirCard-Windows`, commit `8df5b16`.

## Resultado

O log recebido identifica iOS **27.2, build 24B115**. Essa versão está fora da faixa que os mantenedores do AirCard declaram compatível. O projeto depende do Airlift, um método de acesso a arquivos por AirTraffic/Books que foi reportado como bloqueado a partir do **27.2 beta 3**. Um fork do cliente Windows, por si só, não remove esse bloqueio do iOS.

Não foi descoberta nem implementada uma forma alternativa de trocar a imagem no 27.2 RC. Esta alteração corrige a detecção de incompatibilidade; ela não corrige nem contorna o bloqueio dentro do iOS.

## O que o log comprova

1. A conexão com os serviços Apple funciona: dispositivo reconhecido e syslog conectado.
2. Um identificador de cartão é capturado. O nome aparece como `Unknown`, mas isso não explica o timeout posterior.
3. A imagem é processada e as cinco cargas são preparadas pelo cliente.
4. O iPhone responde a `RequestingSync` com `ReadyForSync`. Portanto, o erro antigo de autenticação Grappa do port Windows não aparece nessa tentativa.
5. O último marco registrado é `Finishing Books metadata sync`; após cerca de 120 segundos, o cliente retorna seu timeout genérico.

O código antigo não registra cada chamada privada após esse marco. Portanto, **o log sozinho não identifica a chamada exata que ficou bloqueada, nem prova a causa interna no iOS**. A incompatibilidade declarada pelos mantenedores é a evidência externa que torna o bloqueio do Airlift a principal explicação. O log não mostra uma gravação concluída nem um problema apenas de cache visual.

O `--probe` para em `ReadyForSync`, antes da etapa que movimenta arquivos. Ele pode passar mesmo em um sistema no qual a troca de imagem não funciona.

## O que este fork altera

- Consulta a versão e a build no dispositivo conectado antes de iniciar AFC para operações do Airlift, inclusive limpeza preparatória, exportação do original e movimentação de caches.
- Interrompe em versões incompatíveis com uma mensagem explicativa, em vez de preparar arquivos e aguardar o timeout.
- Preserva as builds conhecidas do 27.2 beta 1 e beta 2: `24B5084k` e `24B5089g`. Não compara builds Apple como números simples: o RC `24B115` não é anterior à beta `24B5099f` só porque tem menos dígitos.
- Exibe o aviso na interface e explica a limitação do Probe.
- Recusa versões que não consegue identificar. Isso evita anunciar suporte sem evidência.

A seleção de versões segue a política publicada pelo projeto, sem garantir que todos os modelos ou operações funcionem nas versões permitidas.

## Como compilar

```powershell
cargo test --locked
cargo build --release --locked
```

O executável fica em `target/release/aircard.exe`. A versão é `1.3.6-compat.1` para distinguir esta compilação de diagnóstico da versão original.

## Validação e limites

Os testes automatizados verificam a classificação do RC informado, das duas betas antigas, da beta 3, da beta 4 e de versões desconhecidas. Eles não executam a gravação no iPhone. Não foi realizado teste físico de troca de imagem no aparelho do usuário.

Para obter suporte real no 27.2 RC seria necessário um novo método de escrita comprovado nessa build, ou outra capacidade de acesso já disponível no aparelho. Aumentar o timeout, mudar a resolução da imagem ou aplicar o mesmo patch de Grappa novamente não demonstra esse acesso. Esta investigação não encontrou uma implementação pública validada para o RC nos projetos consultados.

## Fontes

- [Fork Windows: compatibilidade e correção do handshake](https://github.com/dhava-gautama/AirCard-Windows/tree/8df5b16)
- [Commit que documenta o bloqueio](https://github.com/dhava-gautama/AirCard-Windows/commit/91eb16dd35ccb1f59e9ff9879809115923afc1d8)
- [AirCard macOS: tabela de compatibilidade](https://github.com/Mak5er/AirCard#compatibility)
- [Airlift: funcionamento e diferença entre retorno da API e efeito no arquivo](https://github.com/0xjohnnydev/airlift)
- [CarrierSIM: identificação das builds beta e tratamento da recusa de movimentação](https://github.com/ios-bundles/CarrierSIM/blob/master/carrier.py)

O log original, os identificadores do aparelho e do cartão e as imagens do usuário não fazem parte do código publicado.
