# CL-Vault

## Intro

The CL-vault is a contract that users to move their funds in accordance to given signals by an offchain actor. The user deposits it's funds into the contract. Whenever the range admin submits a new range, the vaults creates a position using that range and deposits all funds in that range.

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

We define the following variables:
$$P_{uc} = min(P_u, P_c)$$
$$P_{lc} = max(P_l, P_c)$$

token0:
$$L=\frac{\Delta x\sqrt{P_{uc}}\sqrt{P_l}}{\sqrt{P_{uc}}-\sqrt{P_l}}$$
token1:
$$L=\frac{\Delta y}{\sqrt{P_u}-\sqrt{P_{lc}}}$$

Let's also explore the three scenarios we can have: $P_c$ is above the range, $P_c$ is below the range, and $P_c$ is within the range.

When $P_c$ is above the range (greater than $P_u$) then we implicitly know that we must swap all token1 for token0 to create a position. We can ignore this case for now.

When $P_c$ is below the range (less than $P_l$) then we implicitly know that we must swap all token0 for token1 to create a position. We can ignore this case for now.

within the range, we can simplify by substituting $P_c$ for $P_{uc}$ and $P_{lc}$ in both liquidity formulas:

Where $P_c$ is the current price, $P_u$ is the upper price, and $P_l$ is the lower price.

token0:
$$L=\frac{\Delta x\sqrt{P_{c}}\sqrt{P_l}}{\sqrt{P_{c}}-\sqrt{P_l}}$$
token1:
$$L=\frac{\Delta y}{\sqrt{P_u}-\sqrt{P_{c}}}$$

By setting these two liquidity formulas equal to each other, we are implicitly stating that the amount of token0 awe are depositiing, will match up correctly with the amount of token1 we are depositing. That gives us this equation:

$$\frac{\Delta x\sqrt{P_{c}}\sqrt{P_l}}{\sqrt{P_{c}}-\sqrt{P_l}}=\frac{\Delta y}{\sqrt{P_u}-\sqrt{P_{c}}}$$

which we can rearrange and solve for $\Delta y$:

$$\Delta y=\Delta x\sqrt{P_{c}}\sqrt{P_l}\frac{\sqrt{P_u}-\sqrt{P_{c}}}{\sqrt{P_{c}}-\sqrt{P_l}}$$

lets define a pool metadata variable $K$:

$$K=\sqrt{P_c}\sqrt{P_l}\frac{\sqrt{P_u}-\sqrt{P_{c}}}{\sqrt{P_{c}}-\sqrt{P_l}}$$
which gives us
$$\Delta y=\Delta xK$$

Now that we have a relationship between what an even deposit would contain (in terms of token0 and token1) we can use this to determine how much token1 we need to swap for token0 to create an even deposit (or vice-versa).

We can use the spot price formula to get the spot price of the pool:

$$P_s=\frac{y}{x}$$

which we can rearrange to get the amount of y tokens we need in terms of x tokens:

$$y=xP_s$$

We now introduce two new variables: $x'$ and $x''$, where $x'$ is the amount of x tokens we will NOT swap, and $x''$ is the amount of x tokens we WILL swap. Of course, implicitly this means we have

$$x=x'+x''$$
which can also be written as
$$x'=x-x''$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta x=\frac{\Delta y}{K}$$

We will replace $\Delta x$ with the amount of tokens we are NOT swapping ($x'$). But how do we replace $\Delta y$? We know that $\Delta y$ is just $\Delta x$ over the spot price, so we can replace $\Delta y$ with $\frac{\Delta x}{P_s}$: and we know that in order to get to $\Delta y$ from $\Delta x$, we will need to swap these tokens. Ah! but we already have a variable that tells us how many tokens we will swap: $x''$. So we can replace $\Delta x$ with $x''$:

$$x'=\frac{x''}{P_sK}$$

We are interested in finding the exact amount of tokens to swap, so let's substitute and solve for $x''$ in terms of x:

$$x-x''=\frac{x''}{P_sK}$$

After some clever algebra, one can verify that we get:

$$x''=\frac{x}{1+\frac{1}{P_sK}}$$

Where $x''$ is the amount of tokens we will swap. and $x$ is the total amount of tokens we have.

### Swapping when we have excess token1

Instead of introducing $x'$ and $x''$, we will introduce $y'$ and $y''$, where $y'$ is the amount of y tokens we will NOT swap, and $y''$ is the amount of y tokens we WILL swap. Of course, implicitly this means we have

$$y=y'+y''$$
which can also be written as
$$y'=y-y''$$

The inverse spot price for x in terms of y is:

$$x=\frac{y}{P_s}$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta y=\Delta xK$$

Where K is the same as above.

We will replace $\Delta y$ with the amount of tokens we are NOT swapping ($y'$). But how do we replace $\Delta x$? We know that $\Delta x$ is just some amount of y tokens multiplied by the spot price, so we can replace $\Delta x$ with $\hat{y}P_s$. Multiplying by the spot price is equivalent to saying, swapping these tokens, but how do we know how many $\hat{y}$ tokens we need to swap?, Ah! but we already have a variable that tells us how many tokens we will swap: $y''$. So we can replace $\hat{y}$ with $y''$:

$$y'=\frac{y''K}{P_s}$$

substituting and solving for the amount of tokens to swap given our total amount of y tokens gives us:

$$y-y''=\frac{y''K}{P_s}$$

After some clever algebra, it is trivial for one to verify that we get:

$$y''=\frac{y}{1+\frac{K}{P_s}}$$

Where $y''$ is the amount of tokens we will swap. and $y$ is the total amount of tokens we have.

### Further/Future optimizations

This current math assumes an ideal pool with infinite liquidity. In reality, we will need to account for slippage.

We can actually create further optimizations here by increasing the amount of tokens to swap by half of the expected slippage, which will lead to returning (not being able to deposit) less tokens.
