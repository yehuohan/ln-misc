#!/usr/bin/env julia

"""
code from:
    https://terrytao.wordpress.com/2019/08/13/eigenvectors-from-eigenvalues/#comment-529038
"""

using LinearAlgebra

N = 10
A = rand(N, N)
A = A + A'

# calculate
λ, V = eigen(A)
VV = zeros(N, N)
for i = 1:N, j = 1:N
    α = setdiff(1:N, j)
    VV[i, j] = prod(λ[i] .- eigvals(A[α, α]))
end
VV ./= [prod(λ[i] - λ[k] for k = 1:N if k != i) for i = 1:N]

println(VV ≈ V'.^2)
display(V'.^2)
println()
display(VV)
