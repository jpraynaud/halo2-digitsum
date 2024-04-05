# <p align="center">Halo2 zk-SNARK "Digit sum" circuit example :lock: </p>

<div align="center">
  <a href='https://github.com/jpraynaud/halo2-digitsum/actions'>
    <img src="https://img.shields.io/github/actions/workflow/status/jpraynaud/halo2-digitsum/ci.yml?label=Tests&style=for-the-badge&branch=main">
  </a>
  <a href='https://github.com/jpraynaud/halo2-digitsum/issues'>
    <img src="https://img.shields.io/github/issues/jpraynaud/halo2-digitsum?label=Issues&style=for-the-badge">
  </a>
  <a href='https://github.com/jpraynaud/halo2-digitsum/network/members'>
     <img src="https://img.shields.io/github/forks/jpraynaud/halo2-digitsum?label=Forks&style=for-the-badge">
  </a>
  <a href='https://github.com/jpraynaud/halo2-digitsum/stargazers'>
    <img src="https://img.shields.io/github/stars/jpraynaud/halo2-digitsum?label=Stars&style=for-the-badge">
  </a>
  <a href='https://github.com/jpraynaud/halo2-digitsum/blob/main/LICENSE'>
    <img src="https://img.shields.io/github/license/jpraynaud/halo2-digitsum?label=License&style=for-the-badge">
  </a>
</div>

## Introduction

This project is a very simple example implementation of a **zk-SNARK** circuit with [Halo 2](https://github.com/zcash/halo2) proof system.

## Goal

In simple words, **Alice** wants to _convince_ **Bob**:
- that she _knows_ a **number** (which can be represented with `8` digits).
- that the **sum of the digits** is equal to a public **number**.
- but she _does not reveal_ such a number to **Bob**.
- and the proof she provides is _succinct_.

