# 🥳 AppFlowy System Design

* Goals of the System
* Some Design Considerations
* High Level Design
* Component Design 

## 🎯 Goals of the System



## 🤔 Some Design Considerations

## 📜 High Level Design

## 📚 Component Design
    
### 📕 Component 1


### 📗 Component 2


### 📘 Flutter Event Flow


### 📙 Rust Event Flow

```
                                                                          ┌─────────┐
                                                                       ┌─▶│Service A│
                                                                       │  └─────────┘
                           ┌─────────┐  ┌───────────┐  ┌─────────────┐ │  ┌─────────┐
                        ┌─▶│Module A │─▶│ Services  │─▶│Deps Resolved│─┼─▶│Service B│
                        │  └─────────┘  └───────────┘  └─────────────┘ │  └─────────┘
                        │                                              │  ┌─────────┐
┌───────┐   ┌────────┐  │  ┌─────────┐                                 └─▶│Service C│
│ Event │──▶│Runtime │──┼─▶│Module B │                                    └─────────┘
└───────┘   └────────┘  │  └─────────┘
                        │
                        │  ┌─────────┐
                        └─▶│Module C │
                           └─────────┘
```

* sync will cause typing lag

