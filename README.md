## ApplyIf

ApplyIf supplies a trait with one method: ``apply_if(cond, closure)``, that applies the closure
on an object if the condition is true, returning the original object otherwise.  
Very useful for the builder pattern when you want to keep the nice ``.builder1().builder2()`` 
chain, and not interrupt it with if-else blocks.  

Comes with a blanket implementation for all sized types.
