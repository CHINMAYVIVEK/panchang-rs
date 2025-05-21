# Panchanga Calculator (पञ्चाङ्ग)

A Rust implementation for calculating the five main elements (Pancha-anga) of the Hindu Astrological Calendar. This tool provides accurate calculations for traditional Hindu astrological elements based on astronomical algorithms and planetary positions.

## Features

Calculates the following elements of Panchanga:
- **Tithi (तिथि)**: Lunar Day
- **Nakshatra (नक्षत्र)**: Lunar Mansion
- **Yoga (योग)**: Luni-Solar Day
- **Karana (करण)**: Half Lunar Day
- **Rashi (राशि)**: Zodiac Sign

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/chinmayvivek/panchang-rs.git
cd panchang-rs
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/panchang-rs`

## Usage

The Panchanga Calculator can be used via the REST API.

### REST API Usage

The Panchanga Calculator also provides a REST API endpoint that accepts HTTP POST requests.

#### API Endpoint

```
POST http://localhost:8080/panchang
```

Request Body (JSON):
```json
{
  "date": "15/08/2023",
  "time": "12:30",
  "zone": "+05:30"
}
```

#### API Example

Using curl:
```bash
curl -X POST "http://localhost:8080/panchang" \
     -H "Content-Type: application/json" \
     -d '{"date":"15/08/2023","time":"12:30","zone":"+05:30"}'
```

Using httpie:
```bash
http POST "http://localhost:8080/panchang" \
     date="15/08/2023" time="12:30" zone="+05:30"
```

Response (JSON):
```json
{
  "tithi": "Prathame, Shukla Paksha",
  "nakshatra": "Pushya",
  "yoga": "Vishkambha",
  "karana": "Bava",
  "rashi": "Simha"
}
```

## Technical Details

- Uses Lahiri's method for Ayanamsa calculations
- Implements simplified VSOP87 algorithm for solar position
- Implements simplified ELP2000 algorithm for lunar position
- Accounts for various periodic perturbations in planetary orbits
- All calculations are based on J2000.0 epoch

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Based on traditional Hindu astrological principles
- Uses modern astronomical algorithms for accurate calculations
- Inspired by various open-source Panchanga calculators