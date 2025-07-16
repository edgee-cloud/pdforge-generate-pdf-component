<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">PDForge PDF Generation component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/pdforge-generate-pdf-component/badge.svg)](https://coveralls.io/github/edgee-cloud/pdforge-generate-pdf-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/pdforge-generate-pdf-component.svg)](https://github.com/edgee-cloud/pdforge-generate-pdf-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/pdforge-generate-pdf)


This component provides a simple way to integrate the Stripe Billing Portal on [Edgee](https://www.edgee.cloud),
served directly at the edge. You map the component to a specific endpoint such as `/pdforge`, and
then you invoke it from your frontend code.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `pdforge.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[components.edge_functions]]
id = "pdforge"
file = "/var/edgee/components/pdforge.wasm"
settings.edgee_path = "/generate-pdf"
settings.api_key = "sk_test_XYZ"
```

### How to use the HTTP endpoint

You can send requests to the endpoint and handle the redirect as follows:

```javascript
// using POST method (returns pdf url as signed)
const response = await fetch('/generate-pdf', {
  method: 'POST',
  body: JSON.stringify({
      "username": "JohnyDoe",
      "email": "John@Doe.com"
  })
});
const data = await response.json();
const a = document.createElement('a');
a.href = data.signedUrl;
a.download = ''; // optional filename hint
document.body.appendChild(a);
a.click();
document.body.removeChild(a);
```

## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)

Build command:
```bash
edgee component build
```

Test command (with local HTTP emulator):
```bash
edgee component test
```

Test coverage command:
```bash
make test.coverage[.html]
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
