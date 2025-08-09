---
mode: ask
type: master-guide
priority: high
tags: [gridtokenx, blockchain, energy-trading, thailand, ddd, rust]
---

# 🚀 GridTokenX Master Development Guide

> **Leading Thailand's Energy Trading Revolution**  
> Expert development guidance for GridTokenX - the pioneering peer-to-peer energy trading blockchain platform transforming Thailand's electricity market.

## 🌟 Project Vision & Context

GridTokenX represents a revolutionary leap in Thailand's energy infrastructure, combining cutting-edge blockchain technology with real-world energy trading:

### 🔋 **Core Innovation**
- **1:1 Token-Energy Ratio**: 1 kWh = 1 GridToken (GT) - absolute transparency
- **Hybrid Consensus**: Proof of Authority (PoA) with Thai energy authorities (EGAT, MEA, PEA)
- **Real-Time Grid Integration**: Live grid monitoring and congestion management
- **Regulatory Compliance**: Full compliance with Thai energy regulations
- **Renewable Focus**: Carbon tracking and sustainability incentives
- **DDD Architecture**: Domain-Driven Design for maintainable, scalable code

### 🏛️ **Authority Ecosystem**
| Authority | Role | Responsibility |
|-----------|------|----------------|
| **EGAT** | Transmission | Electricity Generating Authority of Thailand |
| **MEA** | Distribution | Metropolitan Electricity Authority (Bangkok) |
| **PEA** | Distribution | Provincial Electricity Authority |
| **NEPO** | Policy | National Energy Policy Office |
| **ERC** | Regulation | Energy Regulatory Commission |

## 📋 Available Specialized Prompts

For specific development tasks, leverage these specialized domain-focused prompts:

### 🔧 **Core System Components**
| Prompt | Focus Area | Use Case |
|--------|------------|----------|
| `blockchain-core.prompt.md` | 🔗 Blockchain fundamentals | Blocks, transactions, chain management |
| `consensus-system.prompt.md` | ⚡ Hybrid PoA consensus | Authority node integration, validation |
| `energy-trading.prompt.md` | 🏪 Energy marketplace | Order matching, grid management |
| `p2p-network.prompt.md` | 🌐 Network infrastructure | libp2p, peer discovery, routing |
| `storage-system.prompt.md` | 💾 Data persistence | RocksDB, backup strategies |
| `governance-system.prompt.md` | 🗳️ DAO governance | Voting, regulatory integration |

### 🔌 **Integration & Quality**
| Prompt | Focus Area | Use Case |
|--------|------------|----------|
| `api-development.prompt.md` | 🚀 REST/WebSocket APIs | Authority integrations, real-time feeds |
| `config-utils.prompt.md` | ⚙️ Configuration | Environment management, utilities |
| `testing-qa.prompt.md` | 🧪 Quality assurance | Testing strategies, compliance validation |

## 🇹🇭 Thai Energy Market Intelligence

### 📊 **Market Dynamics**
```
🕘 Peak Hours: 9 AM - 10 PM (premium rates)
🌡️  Seasonal: Hot season (Mar-May) demand surge
🗺️  Grid Zones: Bangkok | Central | North | Northeast | East | West | South
⚡ Grid Standard: 50 Hz with tight tolerance requirements
```

### 📋 **Regulatory Framework**
- ✅ Energy Trading Act B.E. 2562 (2019)
- ✅ Thai Grid Code compliance
- ✅ Real-time reporting requirements
- ✅ License verification for all participants

## 🎯 Development Excellence Principles

### ⚡ **Performance Standards**
| Metric | Target | Critical For |
|--------|--------|--------------|
| **Trading TPS** | 1,000+ | Peak hour operations |
| **Order Latency** | <100ms | Real-time trading |
| **Grid Updates** | <1s | Grid stability |
| **Uptime** | 99.99% | Critical infrastructure |
| **Node Scale** | 10,000+ | Thailand-wide deployment |

### 🔒 **Security Imperatives**
- 🛡️ Cryptographic verification of all energy measurements
- 🔐 Authority node authentication and signature validation
- 🚨 Protection against market manipulation and attacks
- 🔗 Secure integration with Thai grid infrastructure

### ⚖️ **Energy Conservation Laws**
```
Energy Conservation = Input Energy = Output Energy + Transmission Losses
- Real-time validation of energy balance equations
- Prevention of energy double-spending scenarios
- Grid stability constraints enforcement
- Physics-based validation rules
```

## 🚀 Quick Start Development Workflow

### 1. 🎯 **Choose Your Domain**
Select the appropriate specialized prompt based on your current task and domain focus.

### 2. 🇹🇭 **Understand Thai Context** 
Consider regulatory requirements, market characteristics, and cultural factors.

### 3. ⚖️ **Respect Energy Laws**
Ensure all implementations follow energy conservation principles and physics.

### 4. 🏛️ **Authority Integration**
Design for seamless EGAT/MEA/PEA integration and regulatory compliance.

### 5. 🧪 **Test Comprehensively**
Use rigorous testing strategies from `testing-qa.prompt.md` for reliability.

## 🔄 Cross-Component Integration Patterns

### 🔗 **Integration Matrix**
```
┌─────────────────┬─────────────────┬─────────────────┐
│ Blockchain ↔ Energy Trading │ Grid ↔ Consensus │
│ On-chain energy transactions │ Authority privileges │
├─────────────────┼─────────────────┼─────────────────┤
│ P2P ↔ Grid Mgmt │ Storage ↔ All │ API ↔ External │
│ Real-time data │ Persistent data │ Authority APIs │
└─────────────────┴─────────────────┴─────────────────┘
```

### 🎯 **Key Integration Points**
- **Blockchain ↔ Energy Trading**: Energy transactions recorded on-chain with full audit trail
- **Consensus ↔ Authority Integration**: Authority nodes have special consensus privileges
- **P2P ↔ Grid Management**: Real-time grid data propagation through network layer
- **Storage ↔ All Components**: Persistent storage for all system data and state
- **API ↔ External Systems**: Integration endpoints for Thai energy authorities

---

## 🎓 Usage Guidelines

> **Start Here**: Use this master prompt as your foundation, then dive into specialized prompts for detailed implementation guidance.
> 
> **Always Consider**: Thai energy market context, regulatory requirements, and Domain-Driven Design principles in all development decisions.
> 
> **Key Focus**: Maintainable, scalable, and compliant code that serves Thailand's energy transformation.