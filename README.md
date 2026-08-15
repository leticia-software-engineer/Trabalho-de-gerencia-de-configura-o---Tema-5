# Integração,  Entrega e Implantação Contínua(CI/CD)

#### Como as práticas de Integração Contínua, Entrega Contínua e Implantação Contínua automatizam o ciclo de desenvolvimento e disponibilização de software.

---

## 1. Introdução

Atualmente , muito se fala no mundo do desenvolvimento de software, sobre Integração Contínua, Implantação e Entrega Contínua e todo o ciclo que gira em torno do CI/CD. Entender tais conceitos, bem como suas aplicações, se tornou um conhecimento essencial para trabalhar nessa área. Porém, para além de entender é necessário saber aplicar, saber diferenciar, saber criticar e ter um posicionamento crítico, bem como estar por dentro de tudo que permeia essas novas tecnologias.

Pensando nisso, a presente pesquisa busca contextualizar de maneira didática o conhecimento que desenvolvemos acerca das ferramentas e plataformas de CI/CD, suas principais arquiteturas, vantagens e desvantagens, funcionalidades , estratégias de implantação, mecanismos de aprovação, Rollback, monitoramento pós-deploy, cenários de utilização e por último, mas não menos importante, o nosso exemplo prático de um pipeline, exemplificando o funcionamento real e permitindo que você, leitor, experimente também a maneira prática de aplicar esses conceitos.

Para além de uma simples revisão bibliográfica, na presente pesquisa buscamos promover discussões, diferentes perspectivas e abordagens voltadas ao CI/CD e integrar teoria e prática num estudo mais experimental.

---
## 2. Metodologia

O propósito desta pesquisa consiste em fazer uma análise crítica dos conceitos, mecanismos de implantação, plataformas e arquiteturas de CI/CD. Para sua elaboração foi realizada uma pesquisa documental, de caráter exploratório e abordagem mista, mediante a utilização de dados estatísticos, bem como interpretativos, com um desenho transversal no que diz respeito ao tempo. O que buscamos analisar não apenas o que é o CI/CD e discutir o que já se sabe, mas discutir o porquê do que se sabe, discutir vantagens, desvantagens e as implicações das decisões de projeto quando se usa e quando não se usa essas práticas.

---
## 3. Fundamentação teórica

### 3.1. Integração Contínua

A Integração contínua, é a prática de mesclar todas as cópias de trabalho dos desenvolvedores em uma linha principal compartilhada, várias vezes ao dia.[1] Proposta pela primeira vez em 1991, por Grady Booch, inicialmente não definia a necessidade de integração várias vezes ao dia, contudo com o surgimento do Extreming Programming, essa prática passou a ser defendida com mais frequência. Dessa maneira, é incontestável que no contexto atual, muitas coisas mudaram em relação a data de surgimento dessa prática, porém, cada vez mais ela se fez necessária no cotidiano de quem trabalha com desenvolvimento de software. 
 
Assim, integração contínua hoje não é só uma boa prática, mas um pré requisito para um bom desenvolvimento. Isso se dá ao fato de que o desenvolvimento é contínuo, os desenvolvedores trabalham em equipes, muitas vezes até separadas fisicamente, os projetos são longos e as entregas precisam ser frequentes para manter o ritmo ágil exigido pelas empresas e pelos clientes e a maneira mais ágil de integrar esse código é por meio do CI. Com o CI, são feitos muitos commits ao longo do dia e o merge é feito com frequência evitando o máximo de conflitos.

---
### 3.2. Entrega contínua

A Entrega contínua, por sua vez, aborda todo o fluxo de desenvolvimento do projeto de software, visando simplificar o lançamento de entregas de ponta-a-ponta, por meio de feedbacks mais curtos entre os desenvolvedores e seus stakeholders. Estes feedbacks são realizados desde o desenvolvimento do código até a implantação / produção. [2] Tais lançamentos contínuos auxiliam em um melhor suporte e identificação de pontos de melhorias no software, além de minimizar maiores riscos ao identificar os erros precocemente, para serem corrigidos e atualizados em novas versões do sistema, seja em qualquer etapa do projeto.


As etapas de entrega contínua são definidas para promover um ambiente contínuo de homologação e planejamento, para que as entregas sejam devidamente testadas e implementadas da maneira correta. Sendo esses:

- 1.  Configuração do deploy: Criação de ambientes idênticos de teste, homologação e produção, além de ferramentas de automação utilizadas no processo;
     
- 2. Planejamento: Etapa em que são definidas as tarefas à serem desenvolvidas e executadas durante uma sprint;
     
- 3. Execução: Etapa em que tem como objetivo a qualidade do produto que está sendo desenvolvido, no qual é implementados testes contínuos a fim de evitar problemas nas próximas etapas, utilizando ferramentas de verificação  da qualidade;

- 4. Inspeção e Adaptação: Etapa em que executa os testes automatizados, relacionados à Integração Contínua (CI), validando se a entrega atende os requisitos necessários para seguir à próxima etapa;
     
- 5. Operação e Suporte: Nesta etapa, são realizadas implantações automatizadas das entregas, por meio de um ambiente de homologação para testes. Posteriormente, caso tudo estiver nos conformes, é seguido para o ambiente de produção. Caso houver erros no desempenho da aplicação dão resolvidos em uma próxima sprint, ou na sprint atual, a depender da gravidade do erro apresentado.
 
     
---
### 3.3. Implantação contínua

A implantação contínua (CD) é um processo de lançamento de software que usa testes automatizados para validar se as alterações em uma base de código estão corretas e estáveis para implantação autônoma imediata em um ambiente de produção.[3] De um modo geral, trata-se de uma implantação que considera os testes como uma validação automática daquilo que foi produzido e automatiza o processo de deploy, agilizando essa etapa do desenvolvimento. Isso é extremamente útil, quando há um grande volume de alterações em um projeto ou o sistema é muito grande e contém várias entregas frequentes, economizando tempo, trabalho e mão de obra, já que o processo é automático.

---
### 3.4. Diferenciação

Integração contínua, entrega e implantação contínua não é a mesma coisa, mas se complementam. Mas qual a diferença entre elas e porque juntas são tão poderosas?

A diferença principal entre a entrega contínua e a implantação contínua é que a entrega é focada em preparar o código para a implantação, mas deve aguardar algum tipo de aprovação humana antes de ser implementada, enquanto a implantação, automatiza completamente esse processo, assim que uma mudança passa pelos testes, ela é implantada automaticamente.

Por outro lado, a integração contínua é focada em juntar todo o projeto, unindo alterações feitas por todos os membros da equipe diariamente com a maior frequência possível de modo a manter o sistema sempre sincronizado e atualizado por todos e para todos, agilizando o processo posterior da entrega contínua. Então o CI é um acelerador do CD, é um processo que ao ser integrado ao CD, agiliza todo o desenvolvimento do software e eles se complementam como boas práticas.

Logo, vemos que o CI e o CD são práticas modernas e muito requisitadas para quem deseja trabalhar na área de TI e por isso neste estudo buscamos aprofundar uma discussão a respeito dessas práticas de modo a consolidar o seu entendimento acerca desse tema.

---
## 4. Ferramentas e plataformas de CI/CD

Depois de escolher utilizar CI/CD é preciso saber como utilizar e para isso é necessário ter o conhecimento das ferramentas que dão suporte a essas práticas e é dessas ferramentas que vamos falar agora, fazendo uma análise comparativa do que está disponível, quais as vantagens e desvantagens dessas ferramentas e o que é qual a melhor ferramenta para o seu projeto.

---

### 4.1. Análise comparativa

- a. **Requisitos de Visão Plataforma ALM:** Requisitos, testes, riscos, defeitos, arquitetura, linhas de base e evidências de auditoria estão todos interligados em um único sistema controlado. Essa ferramenta é útil para quando se deseja acompanhar o processo de ponta a ponta considerando toda a engenharia do software, principalmente a rastreabilidade dos requisitos. Essa ferramenta possui alta incorporação de IA para acompanhamento dos processos. As principais vantagens estão relacionadas ao controle e rastreabilidade facilitando auditoria, sendo interessante para projetos que envolvem, por exemplo, verbas governamentais e necessitam de auditoria frequente. Enquanto algumas desvantagens são o custo elevado , pensando no orçamento e a implementação é lenta pois necessita de muita especificação. 

- b.**Jenkins**: O Jenkins funciona como um servidor de automação de integração contínua e entrega contínua (CI/CD), ajudando os desenvolvedores a criar, testar e implantar aplicativos com eficiência. Em sua essência, o Jenkins segue uma abordagem orientada a pipelines.[4] O Jenkins é uma das ferramentas de CI/CD mais utilizadas por possuir alta flexibilidade e escalabilidade, possui alto ecossistema de plugins e uma vasta comunidade de desenvolvedores que o utilizam, facilitando a resolução de problemas. Entretanto, a sua configuração exige um alto custo de manutenção, em aplicações de larga escala ele não é muito recomendado, pois podem surgir gargalos de desempenho.

- c. **CI / CD do GitLab**: O GitLab CI é usado para automatizar e acelerar as etapas de teste, construção, implantação e entrega de sistemas. Enquanto o GitLab CD permite que desenvolvedores de software automatizem o processo de implantação de seus aplicativos em ambientes diversos. Algumas vantagens dessa ferramenta são, por exemplo, o CI baseado em chat, vinculação do Docker, conexão com repositórios externos, runners CI dimensionáveis automaticamente. Mas ao mesmo tempo vários desses recursos só estão disponíveis na versão premium e possuem um custo financeiro associado, e muitas vezes é percebida uma certa dificuldade para migrar um projeto da versão gratuita para a premium. Mas, considerando projetos de grande escala, essa ferramenta é mais vantajosa do que o jenkins e costuma apresentar alta confiabilidade.

- d. **GitHub Actions**: É uma ferramenta que permite automatizar, personalizar e executar fluxos de trabalho de desenvolvimento de software diretamente no repositório do Github. Ele pode ser usado para implementar tanto CI quanto CD. O GitHub fornece máquinas virtuais do Linux, Windows e macOS para a execução dos seus fluxos de trabalho e possui como componentes principais os fluxos de trabalho (workflows), eventos, trabalhos e ações.[5] As principais vantagens dessa ferramenta são voltadas a simplicidade pois ela é integrada ao Github que já é muito utilizado pelos desenvolvedores para armazenamento de código, também oferece suporte a diversos sistemas operacionais e linguagens de programação e possui uma comunidade vasta de desenvolvedores. E uma das vantagens dele que se sobrepõe às ferramentas mencionadas anteriormente é justamente o potencial do plano gratuito que permite uma ampla gama de recursos para projetos open source.


- e. **Pipelines Bitbucket**: o Bitbucket é um sistema de versionamento distribuído baseado no Git. Possui integração total com outros produtos da Atlassian, como Jira, Fisheye e Bamboo. Uma das maiores vantagens que a Bitbucket tem sobre seus concorrentes é oferecer um número ilimitado de repositórios privados. Pipelines Bitbucket é um serviço integrado de CI/CD embutido na Nuvem Bitbucket. Então sabemos que o bitbucket funciona como o github porém possui suas peculiaridades. Mas como os outros também possui suas desvantagens, uma desvantagem clara é voltada a limitação com suporte apenas a redes IPV4, além dos minutos de build serem limitados, então para projetos muitos grandes com build longos o bitbucket pipelines não é a melhor opção. 

- f. **AWS Code Pipeline**: Essa ferramenta é integrada com a plataforma de armazenamento em uvem da AWS e permite que o processo de CI e CD seja feito nesse mesmo ambiente possui armazenamento de artefatos baseado em S3 e a infraestrutura é gerenciada pela própria AWS, sendo uma boa escolha para projetos grandes ou que já utilizam a AWS. Algumas desvantagens dessa ferramenta são principalmente voltadas a interface ser pouco intuitiva e alguns problemas de usabilidade como a latência e também o forte acoplamento com a AWS, o que dificulta bastante uma migração para outra plataforma.

---

## 5. Arquiteturas 

A Arquitetura de CI/CD refere-se à organização estrutural e operacional dos sistemas que automatizam a compilação, teste, empacotamento e implantação de software. Ela define como o código flui do repositório até o ambiente final de produção, impactando diretamente a segurança, escalabilidade, tempo de execução e a superfície de ataque da infraestrutura. Com o passar do tempo, as arquiteturas evoluíram de servidores simples para modelos mais avançados, distribuídos e nativos da nuvem.

### 5.1. Modelos arquiteturais de CI/CD:

Arquitetura Push-Based: O servidor de CI/CD (ou runner) escuta eventos do repositório (via webhooks), executa o build, roda os testes e se conecta diretamente ao ambiente de destino para aplicar as alterações.
> - Fluxo: Git Commit ➔ Servidor CI/CD ➔ Autenticação Externa ➔ Push para Produção
> - Exemplos: GitHub Actions, GitLab CI, Jenkins, CircleCI, AWS CodePipeline.
> - Vantagens: Configuração inicial rápida, suporta múltiplos alvos (SaaS, VMs, Bare Metal), visibilidade centralizada do pipeline
> - Desvantagens: Exige expor portas ou credenciais de produção para o CI, risco de gargalo se os runners forem compartilhados
> - Caso de uso ideal: Projetos heterogêneos, monorepos, arquiteturas baseadas em VMs e funções Serverless. 

Arquitetura Pull-Based (GitOps): Nenhum sistema externo possui credenciais de acesso ao ambiente final. Em vez disso, um agente ou operador interno (instalado dentro do cluster de destino) monitora continuamente o repositório Git e puxa (pull) o estado desejado, aplicando-o localmente.
> - Fluxo: Git Commit ➔ Agente Interno (In-Cluster) ➔ Sincronização Local
> - Exemplos: Argo CD, Flux CD, Fleet.
> - Vantagens: Sem credenciais expostas para fora, detecção automática de drift (desvio de configuração), rollback trivial apontando para um commit anterior
> - Desvantagens: Exige que a infraestrutura seja 100% declarativa, curva de aprendizado alta, restrito principalmente a contêineres e Kubernetes
> - Caso de uso ideal: Ambientes nativos em Kubernetes, arquiteturas de microsserviços altamente reguladas. 

Arquitetura Efêmera e Serverless (Kubernetes-Native): Não existem executores (runners) fixos ligados o tempo todo. Cada etapa do pipeline dispara a criação de um pod ou contêiner temporário isolado em um cluster, que é destruído imediatamente após a conclusão.
> - Fluxo: orientado a eventos e o ciclo de vida da própria infraestrutura faz parte do fluxo. O pipeline não se conecta a um servidor existente, ele cria o servidor, executa o trabalho e destrói o servidor. 
> - Exemplos: Tekton Pipelines, Jenkins X.
> - Vantagens: Isolamento total entre jobs (sem estado residual), uso eficiente de recursos (paga/aloca apenas quando executa), escalabilidade horizontal ilimitada
> - Desvantagens: Cold start (atraso para subir imagens do runner), gestão complexa de cache entre execuções
> - Caso de uso ideal: Grandes organizações com centenas de deploys diários e necessidade de isolamento estrito.

---

## 6. Funcionalidades
7. Estratégias de implantação
8. Mecanismos de aprovação


- 9. Rollback

10


## 10. Monitoramento pós-deploy

11


## 11. Cenários de utilização

12


## 12. Nosso exemplo prático

13


- 13. Conclusão

14

---

### Equipe
---
Gabriel Souza Santos

Herberthy Samir Oliveira F. de Souza

Joaquim Arthur Muniz Leite

José Dhonatan Fernandes de Almeida

José Welton de Sousa Melo 

Letícia Maria dos Santos Dias

Sarah Mendes Teles

---
## 14. Referências

[1] WIKIPEDIA CONTRIBUTORS. Integração contínua. Disponível em: <https://pt.wikipedia.org/w/index.php?title=Integra%C3%A7%C3%A3o_cont%C3%A Dnua&oldid=60895845>. [URL 🔗](https://pt.wikipedia.org/w/index.php?title=Integra%C3%A7%C3%A3o_cont%C3%ADnua&oldid=60895845)

[2] DE SOUSA, Sabryna et al. Entrega Contínua de Software: Um estudo de caso. [S.l.: S.n.]. Disponível em: <https://encurtador.com.br/mOYO>. Acesso em: 13 de agosto, 2026. [URL 🔗](https://encurtador.com.br/mOYO)

[3] ATLASSIAN. O que é implantação contínua? Disponível em: <https://www.atlassian.com/br/continuous-delivery/software-testing/continuous-deplo yment>. Acesso em: 13 ago. 2026. [URL 🔗](https://www.atlassian.com/br/continuous-delivery/software-testing/continuous-deployment)

[4] What is Jenkins? A Guide to CI/CD. Cloudbees.comCloudBees, , 10 dez. 2025. Disponível em: <https://www.cloudbees.com/blog/what-is-jenkins>. Acesso em: 15 ago. 2026 [URL 🔗](https://www.cloudbees.com/blog/what-is-jenkins)

[5] GitHub Actions: o que é, como funciona e boas práticas! Disponível em: <https://focusnfe.com.br/blog/github-actions/>. Acesso em: 15 ago. 2026. [URL 🔗](https://focusnfe.com.br/blog/github-actions/)
