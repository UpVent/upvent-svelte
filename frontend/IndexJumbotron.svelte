<script>
    import {
        Button,
        ButtonGroup,
        Col,
        Container,
        Row,
    } from 'sveltestrap';

</script>

<Container>
    <Row>
        <Col xs="auto">
            <h1 class="display-4 fw-bold">Nube inteligente para negocios inteligentes.</h1>
            <p class="small text-muted fs-5">Somos una compañía de software empresarial, basados en un modelo
                de desarrollo de software libre. En UpVent construimos el futuro
                del software libre, resolviendo problemas y otorgando un sentido
                de libertad a nuestros clientes.
            </p>
            <Row>
                <Col xs="auto">
                    <ButtonGroup class="shadow-sm">
                        <Button color="primary" >Comenzar</Button>
                        <Button active color="dark">Explorar Software</Button>
                    </ButtonGroup>
                </Col>
            </Row>
        </Col>
        <Col xs="auto">.col-auto - variable width content</Col>
    </Row>

</Container>
