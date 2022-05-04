<!--
order: 2
-->

# State

Intergamm is a stateless in the sense that there is no business state stored in a KV store. It acts as a pass-through / client towards other chains via IBC.

Some timeout / retry related data might be stored in the future for robustness and correctness purposes.
