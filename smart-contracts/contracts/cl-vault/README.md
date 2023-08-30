# CL-Vault

# Testing

The main effort for testing is done using test tube, this means that any code coverage tools won't pick up on lines covered.
Some functions should still have their own unit tests/prop tests ofcourse, especially if the intended behaviour might not be clear later.

## [Test-tube](https://github.com/osmosis-labs/test-tube)

### Running tests

To run any test tube tests, a wasm file of the contract needs to be built for test tube. After which the tests can be ran.
To build the wasm file, run `cargo test-tube-build`. after the build succeeds, tests can be ran using `cargo test-tube`.

### Writing tests

To make use of our current setup, all tests using test-tube need to go in `src/test_tube`. `test_tube/initialize.rs` contains some helpers to setup
a Concentrated Liqudity pool and instantiate the cl-vault contract. Any further behaviour tests need to be written potentially in a new file depending on the size of the test

## Unit testing

For unit testing coverage, it is up to the reviewers to decide where that level of granularity is needed, or where just test tube provides adequate testing

## Single sided deposit

Assume you want to enter a concentrated liquidity pool at a certain range. The pool is an OSMO/DAI pool and you want to enter the pool by just depositing OSMO. How will the contract know how much OSMO to swap for DAI before depositing into the pool?

The current chain-side concentrated liquidity deposits support lopsided deposits, and will refund any extra tokens, but will only support one-sided deposits on extreme tick ranges. This routine solves this problem.

### Swapping when we have excess token0

We start with the two concentrated liquidity-needed formulas:

token0:
$$L=\frac{\Delta x\sqrt{P_u}\sqrt{P_l}}{\sqrt{P_u}-\sqrt{P_l}}$$
token1:
$$L=\frac{\Delta y}{\sqrt{P_u}-\sqrt{P_l}}$$

If we also equate the two Liquidity needed formulas, we get a cleaner relationship between $\Delta x$ and $\Delta y$:

$$\frac{\Delta x\sqrt{P_u}\sqrt{P_l}}{\sqrt{P_u}-\sqrt{P_l}}=\frac{\Delta y}{\sqrt{P_u}-\sqrt{P_l}}$$

which we can simplify by multiplying both sides by $\sqrt{P_u}-\sqrt{P_l}$:

$$\Delta x\sqrt{P_u}\sqrt{P_l}=\Delta y$$

We can now easily get how much $\Delta y$ we need in terms of $\Delta x$:

$$\Delta y=\Delta x\sqrt{P_u}\sqrt{P_l}$$

and also how much $\Delta x$ we need in terms of $\Delta y$:

$$\Delta x=\frac{\Delta y}{\sqrt{P_u}\sqrt{P_l}}$$

the only problem is we only have x tokens, no y tokens. so we need to understand how many x tokens to swap to give us the exact amount of y tokens we need.

We can use the spot price formula to get the spot price of the pool:

$$P_s=\frac{x}{y}$$
_todo: check me to make sure this fraction is correct_

which we can rearrange to get the amount of y tokens we need in terms of x tokens:

$$y=\frac{x}{P_s}$$

We now introduce two new variables: $x'$ and $x''$, where $x'$ is the amount of x tokens we will NOT swap, and $x''$ is the amount of x tokens we WILL swap. Of course, implicitly this means we have

$$x=x'+x''$$
which can also be written as
$$x'=x-x''$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta x=\frac{\Delta y}{\sqrt{P_u}\sqrt{P_l}}$$

We will replace $\Delta x$ with the amount of tokens we are NOT swapping ($x'$). But how do we replace $\Delta y$? We know that $\Delta y$ is just $\Delta x$ over the spot price, so we can replace $\Delta y$ with $\frac{\Delta x}{P_s}$: and we know that in order to get to $\Delta y$ from $\Delta x$, we will need to swap these tokens. Ah! but we already have a variable that tells us how many tokens we will swap: $x''$. So we can replace $\Delta x$ with $x''$:

$$x'=\frac{x''}{P_s\sqrt{P_u}\sqrt{P_l}}$$

We are interested in finding the exact amount of tokens to swap, so let's substitute and solve for $x''$ in terms of x:

$$x-x''=\frac{x''}{P_s\sqrt{P_u}\sqrt{P_l}}$$

After some clever algebra, one can verify that we get:

$$x''=\frac{xP_s\sqrt{P_u}\sqrt{P_l}}{1+P_s\sqrt{P_u}\sqrt{P_l}}$$

to make smart contract math more fun, lets define a pool metadata variable $K$:

$$K=P_s\sqrt{P_u}\sqrt{P_l}$$

which turns our equation into:

$$x''=\frac{xK}{1+K}$$

Where $x''$ is the amount of tokens we will swap. and $x$ is the total amount of tokens we have.

### Swapping when we have excess token1

Instead of introducing $x'$ and $x''$, we will introduce $y'$ and $y''$, where $y'$ is the amount of y tokens we will NOT swap, and $y''$ is the amount of y tokens we WILL swap. Of course, implicitly this means we have

$$y=y'+y''$$
which can also be written as
$$y'=y-y''$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta y=\Delta x\sqrt{P_u}\sqrt{P_l}$$

We will replace $\Delta y$ with the amount of tokens we are NOT swapping ($y'$). But how do we replace $\Delta x$? We know that $\Delta x$ is just some amount of y tokens multiplied by the spot price, so we can replace $\Delta x$ with $\hat{y}P_s$. Multiplying by the spot price is equivalent to saying, swapping these tokens, but how do we know how many $\hat{y}$ tokens we need to swap?, Ah! but we already have a variable that tells us how many tokens we will swap: $y''$. So we can replace $\hat{y}$ with $y''$:

$$y'=y''P_s\sqrt{P_u}\sqrt{P_l}$$

substituting and solving for the amount of tokens to swap given our total amount of y tokens gives us:

$$y''=\frac{y}{(1+P_s\sqrt{P_u}\sqrt{P_l})}$$
or, given our pool metadata constant from earlier:
$$y''=\frac{y}{(1+K)}$$
Where $y''$ is the amount of tokens we will swap. and $y$ is the total amount of tokens we have.

### Further/Future optimizations

This current math assumes an ideal pool with infinite liquidity. In reality, we will need to account for slippage.

We can actually create further optimizations here by increasing the amount of tokens to swap by half of the expected slippage, which will lead to returning (not being able to deposit) less tokens.
