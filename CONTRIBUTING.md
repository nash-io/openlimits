# How to contribute

I'm really glad you're reading this, because we need all the industry to work together in good standards.

## Submitting changes

Please send a [GitHub Pull Request](https://github.com/nash-io/openlimits/pull/new/master) with a clear list of what you've done (read more about [pull requests](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests)). When you send a pull request, we will love you forever if it is small. Incremental improvements accumulate fast.

### Pull Request rules

Every Pull Request should have one `release label` so the auto-release action can correctly bump the semantic version (`major.minor.patch`). A `release label` can be:

* `major`: Used when you make incompatible API changes,
* `mijor`: Used when you add functionality in a backwards-compatible manner, and
* `patch`: Used when you make backwards-compatible bug fixes.

For more details about it, check the [semver](https://semver.org/) website.

As a reminder: According to the [spec-item-4](https://semver.org/#spec-item-4): Major version zero (0.y.z) is for initial development. Anything MAY change at any time. The public API `SHOULD NOT` be considered stable.

## Coding conventions

The conventions still needs to be defined, for now we rely on the rust-analyzer.

##### Commenting

Notes should be in the format `// TYPE:`, where `TYPE` is:

  * `FIXME` - for parts of the code that needs to be fixed,
  * `TODO` - for incomplete code, or code that needs to be improved in some way or
  * `REVIEW` - for starting a discussion with the code reviewers. This should never be merged into the main branch.

##### Unwrapping
  * Try your best to never make use of `unwrap` and `expect` in the library code. The errors must be propagated or correctly handled.
  * If you really need to make use of `expect`, make sure to add a comment explaining the constraints that assure it will never fail.
  * Always make use of `expect` with a proper message instead of `unwrap`.

##### Logging
  * It's OK to use `println!` for quick debugging on the development side, but they shouldn't be pushed to remote.

## Licensing

When you contribute to this repository you are transferring any rights claim over the contributed material to Neon Exchange AG to the OpenLimits project under BSD-2 license. Neon Exchange AG commits that this project will only ever be available under BSD-2 or MIT licenses, forever being open-source.

Thanks,
Fabio C. Canesin
