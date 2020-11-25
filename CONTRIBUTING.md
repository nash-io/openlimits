# How to contribute

I'm really glad you're reading this, because we need all the industry to work together in good standards.

## Submitting changes

Please send a [GitHub Pull Request](https://github.com/nash-io/openlimits/pull/new/master) with a clear list of what you've done (read more about [pull requests](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests)). When you send a pull request, we will love you forever if it is small. Incremental improvements accumulate fast.

## Coding conventions

The conventions still needs to be defined, for now we rely on the rust-analyzer.

##### Unwrapping
  * Try your best to never make use of `unwrap` and `expect` on production code. The errors must be propagated or correctly handled.
  * When writing test codes, always make use of `expect` with a proper message instead of `unwrap`.

##### Logging
  * It's OK to use `println!` for quick debugging on the development side, but they shouldn't be pushed to remote.

## Licensing

When you contribute to this repository you are transferring any rights claim over the contributed material to Neon Exchange AG to the OpenLimits project under BSD-2 license. Neon Exchange AG commits that this project will only ever be available under BSD-2 or MIT licenses, forever being open-source.

Thanks,
Fabio C. Canesin
