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
$$L=\frac{\Delta x\sqrt{P_{c}}\sqrt{P_u}}{\sqrt{P_{u}}-\sqrt{P_c}}$$
token1:
$$L=\frac{\Delta y}{\sqrt{P_c}-\sqrt{P_{l}}}$$

By setting these two liquidity formulas equal to each other, we are implicitly stating that the amount of token0 awe are depositiing, will match up correctly with the amount of token1 we are depositing. That gives us this equation:

$$\frac{\Delta x\sqrt{P_{c}}\sqrt{P_u}}{\sqrt{P_{u}}-\sqrt{P_c}}=\frac{\Delta y}{\sqrt{P_c}-\sqrt{P_{l}}}$$

which we can rearrange and solve for $\Delta y$:

$$
\Delta y=\Delta x(\sqrt{P_{c}}-\sqrt{P_l})
\frac{\sqrt{P_u}\sqrt{P_{c}}}{\sqrt{P_{l}}-\sqrt{P_c}}
$$

lets define a pool metadata variable $K$:

$$
K=(\sqrt{P_{c}}-\sqrt{P_l})
\frac{\sqrt{P_u}\sqrt{P_{c}}}{\sqrt{P_{l}}-\sqrt{P_c}}
$$

which gives us
$$\Delta y=\Delta xK$$

Now that we have a relationship between what an even deposit would contain (in terms of token0 and token1) we can use this to determine how much token1 we need to swap for token0 to create an even deposit (or vice-versa).

We can use the spot price formula to get the spot price of the pool:

$$P_c=\frac{y}{x}$$

which we can rearrange to get the amount of y tokens we need in terms of x tokens:

$$y=xP_c$$

We now introduce two new variables: $x'$ and $x''$, where $x'$ is the amount of x tokens we will NOT swap, and $x''$ is the amount of x tokens we WILL swap. Of course, implicitly this means we have

$$x=x'+x''$$
which can also be written as
$$x'=x-x''$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta x=\frac{\Delta y}{K}$$

We will replace $\Delta x$ with the amount of tokens we are NOT swapping ($x'$). But how do we replace $\Delta y$? We know that $\Delta y$ is just $\Delta x$ multiplied by the spot price, so we can replace $\Delta y$ with $\Delta xP_c$: and we know that in order to get to $\Delta y$ from $\Delta x$, we will need to swap these tokens. Ah! but we already have a variable that tells us how many tokens we will swap: $x''$. So we can replace $\Delta x$ with $x''$:

$$x'=\frac{x''P_c}{K}$$

We are interested in finding the exact amount of tokens to swap, so let's substitute and solve for $x''$ in terms of x:

$$x-x''=\frac{x''P_c}{K}$$

After some clever algebra, one can verify that we get:

$$x''=\frac{x}{1+\frac{P_c}{K}}$$

Where $x''$ is the amount of tokens we will swap. and $x$ is the total amount of tokens we have.

### Swapping when we have excess token1

Instead of introducing $x'$ and $x''$, we will introduce $y'$ and $y''$, where $y'$ is the amount of y tokens we will NOT swap, and $y''$ is the amount of y tokens we WILL swap. Of course, implicitly this means we have

$$y=y'+y''$$
which can also be written as
$$y'=y-y''$$

The inverse spot price for x in terms of y is:

$$x=\frac{y}{P_c}$$

We are now looking to swap such that the following relationship is satisfied:

$$\Delta y=\Delta xK$$

Where K is the same as above.

We will replace $\Delta y$ with the amount of tokens we are NOT swapping ($y'$). But how do we replace $\Delta x$? We know that $\Delta x$ is just some amount of y tokens over by the spot price, so we can replace $\Delta x$ with $\frac{\Delta{y}}{P_c}$. This is equivalent to saying, swap these tokens, but how do we know how many $\Delta{y}$ tokens we need to swap?, Ah! but we already have a variable that tells us how many tokens we will swap: $y''$. So we can replace $\Delta{y}$ with $y''$:

$$y'=\frac{y''K}{P_c}$$

substituting and solving for the amount of tokens to swap given our total amount of y tokens gives us:

$$y-y''=\frac{y''K}{P_c}$$

After some clever algebra, it is trivial for one to verify that we get:

$$y''=\frac{y}{1+\frac{K}{P_c}}$$

Where $y''$ is the amount of tokens we will swap. and $y$ is the total amount of tokens we have.

and let us not forget that

$$K=\sqrt{P_c}\sqrt{P_l}\frac{\sqrt{P_u}-\sqrt{P_{c}}}{\sqrt{P_{c}}-\sqrt{P_l}}$$

### Further/Future optimizations

This current math assumes an ideal pool with infinite liquidity. In reality, we will need to account for slippage.

We can actually create further optimizations here by increasing the amount of tokens to swap by half of the expected slippage, which will lead to returning (not being able to deposit) less tokens.

# Multi-position support

## intro

The CL vault supports multiple positions internally.

### position ratios

Each position is saved in the contract with a ratio. This ratio dictates how many tokens are sent to a position when a user deposits.

Given any current spot price, each position needs a different amount of tokens. Lets take 3 positions $P_1$, $P_2$ and $P_3$. at some price $P$, $P_1$ needs 30% of token0 and 70% of token1. $P_2$ needs 45% token0 and 55% token1 and $P_3$ 80% token0 and 20% token1. Effectively this means that each position has the following internal ratio:

$$
IR_{P_1} = \frac{3}{7}\\
IR_{P_2} = \frac{45}{55}\\
IR_{P_3} = \frac{80}{20}
$$

lets say that we allocate 20% of tokens to $P_1$, 50% of tokens to $P_2$, and 30% of all tokens to $P_3$. When we want to know how many total tokens we sent to a position, we multiply the internal ratio with the external ratio.
For actual ratios, we then get

$$
R_{P_1} = IR_{P_1} \cdot 0.20 = \frac{3}{7} \cdot \frac{1}{5} = \frac{3}{35} \\
R_{P_2} = IR_{P_2} \cdot 0.50 = \frac{45}{55} \cdot \frac{1}{2} = \frac{45}{110} = \frac{9}{22} \\
R_{P_3} = IR_{P_3} \cdot 0.30 = \frac{80}{20} \cdot \frac{3}{10} = \frac{240}{200} = \frac{6}{5}
$$

Our effective ratio then becomes

$$
\frac{3}{35} + \frac{9}{22} + \frac{6}{5} = \frac{261}{154}
$$

So for every 261 token0, we need 154 token1

### Lowering ratio

aka the LowerRatio message. This message is used to decrease the allocation of funds towards a specific range.

#### Inputs

- position id
- old ratio: required because we may have changed actual ratios due to impermanent loss
- new ratio

#### Logic

this does not actually do any rebalancing. It just updates the internal ratio of a position. The actual ratio will be updated when **\*\*\*\***\_**\*\*\*\***

### increasing ratio

aka the AddRatio message. This message is used to increase the allocation of funds towards a specific range.

#### Inputs

- position id
- old ratio: required because we may have changed actual ratios due to impermanent loss
- new ratio

#### Logic

this does not actually do any rebalancing. It just updates the internal ratio of a position. The actual ratio will be updated when **\*\*\*\***\_**\*\*\*\***

### Increasing a positions funds

aka the IncreaseFunds message. This message is used to increase the allocation of funds towards a specific range.

#### Inputs

- position id
- amount of token0
- amount of token1

#### Logic

1. this simply gets the current postion by id
2. creates a position with an identical upper and lower tick
3. uses Replies::RangeAddToPosition to merge the two positions together

### Decreasing a positions funds

aka the DecreaseFunds message. This message is used to decrease the allocation of funds towards a specific range.

#### Inputs

- position id
- liquidity amount to reduce by

#### Logic

1. this just does a partial withdraw on a position. Leftover funds are stored as unused funds in the contract

### Moving a position

Moving a position is fairly straightforward, since we only need to move the position from one range to another.

#### Inputs

- old position id
- new lower price
- new upper price
- max slippage

#### Logic

1. get the current position
2. save this position in modify_range_state - this lets us rejoin the flow that we already have for entering the position with the reply handler of a withdraw message
3. withdraw the current posiition fully
4. return control to the main flow that handles position entry after a withdraw position message

### Creating and deleting positions

Deleting a position means all

### Rebalancing over all positions

TODO

## Deposits into multi-range vaults

### Step 1: get_min_ratio_per_position

first we call the get_min_ratio_per_position function with the positions we have and the spot price.

This function will take an arbitrary liquidity amount and get the ratio that each position would give given that current spot price. we then do one last step which normalizes the rations against each other.

we then return a vec of potitions and their respective ratios.

[notes for improvement]
We currently use an arbitrary liquidity amount, which could concievably lead to low-liquidity or narrow-tick edge cases.

It would be better to outline the formulas for amount_0 and amount_1 in the three possible scenarios:

when the current price is above the range
when the current price is below the range
when the current price is within the range

and then we will _ideally_ be able to divide the two equations by each other, this would cancel out the liquidity amount and leave us directly with a ratio that we can return.

It is not a huge improvement, but it may avoid some issues in the future.
