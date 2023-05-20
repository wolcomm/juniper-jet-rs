# Examples

To run the examples, a device running the appropriate version of Junos
should be available, and configured to enable the JET gRPC server endpoint.

For convenience a set of RSA keys and associated X.509 certificates are
provided in `examples/pki/`. These are for testing purposes only and should
**never** be used for production deployments.

## Router configuration

Minimally, the device should be configured with the appropriate server TLS
certificate.

First, copy the PEM encoded server certificate and key bundle onto the device:

``` bash
$ scp examples/pki/server.bundle.pem $JUNOS_DEVICE_ADDR:/var/tmp/
```

And load the file contents into the configuration:

``` junos
[edit]
support@router# set security certificates local jet-example load-key-file /var/tmp/server.bundle.pem
```

The server X.509 certificate contains a Subject Alternative Name extension
containing `router.example.net`.

Then configure the JET service to use the certificate:

``` junos
[edit system services extension-service]
support@router# show
request-response {
    grpc {
        ssl {
            local-certificate jet-example;
            mutual-authentication {
                certificate-authority ca;
                client-certificate-request request-certificate-and-verify;
            }
        }
        max-connections 8;
        routing-instance mgmt_junos;
    }
}
```

To avoid having verification fail without hacking at your local DNS
infrastructure, the example application provides a CLI option
`--server-tls-domain-name` that will override the name used during server
certificate verification.

### Mutual TLS authentication

Optionally, to enable mutual TLS authentication, the Junos device should be
configured to verify the presented client certificate using the appropriate CA
certificate.

Transfer the CA X.509 certificate to the device:

``` bash
$ scp examples/pki/ca.crt $JUNOS_DEVICE_ADDR:/var/tmp/
```

Configure a `ca-profile` for the CA:

``` junos
[edit security pki]
support@router# show
ca-profile ca {
    ca-identity ca;
}
```

And load the file contents into configured `ca-profile`:

``` junos
support@router> request security pki ca-certificate load ca-profile ca filename /var/tmp/ca.crt
```

Then configure the JET service to require client certificates and to verify
using the correct CA:

``` junos
[edit system services extension-service request-response grpc ssl]
support@router# show
local-certificate jet-example;
mutual-authentication {
    certificate-authority ca;
    client-certificate-request require-certificate-and-verify;
}
```

The example application can then be run using `--client-cert-path
examples/pki/client.crt` and `--client-key-path examples/pki/client.key`
options.
