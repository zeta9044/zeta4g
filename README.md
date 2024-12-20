# Zeta4g

Zeta4g is a graph database implementation with Neo4j Cypher query compatibility, written in Python.

## Features

- Graph database core functionality
- Neo4j Cypher query compatibility
- Memory-based and persistent storage options
- High-performance indexing
- ACID transaction support

## Installation

```bash
pip install zeta4g
```

Or with Poetry:

```bash
poetry add zeta4g
```

## Quick Start

```python
from zeta4g import GraphDatabase

# Create a new database instance
db = GraphDatabase()

# Create nodes
alice = db.create_node(labels={"Person"}, properties={"name": "Alice"})
bob = db.create_node(labels={"Person"}, properties={"name": "Bob"})

# Create relationship
db.create_relationship("KNOWS", alice.id, bob.id, properties={"since": 2023})
```

## Development

### Prerequisites

- Python 3.9+
- Poetry

### Setting up the development environment

```bash
# Clone the repository
git clone https://github.com/yourusername/zeta4g.git
cd zeta4g

# Install dependencies
poetry install

# Activate the virtual environment
poetry shell

# Run tests
pytest
```

### Running Tests

```bash
# Run all tests
pytest

# Run with coverage report
pytest --cov=zeta4g
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add some amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Project Status

This project is currently in active development. The API may change without notice until version 1.0.0 is released.

## Acknowledgments

- [Neo4j](https://neo4j.com/) for their Cypher query language specification
- All contributors who have helped with code, bug reports, and suggestions
