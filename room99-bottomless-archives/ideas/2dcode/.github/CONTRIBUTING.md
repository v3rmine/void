# Contributing
We expect contributors to abide by our underlying [code of conduct](CODE_OF_CONDUCT.md). All conversations and discussions on GitHub (issues, pull requests) must be respectful and harassment-free.

Remember that communication is the lifeblood of any Open Source project. We are all working on this together, and we are all benefiting from this software. It's very easy to misunderstand one another over asynchronous, text-based conversations: When in doubt, assume everyone you're interacting within this project has the best intentions.

If you feel another member of the community has violated our Code of Conduct, you may contact the repository owner at {Author.mail}.

## Where to contribute

All issues labeled with
- `Info: Good first issue` are issues meant for newer developers.
- `Status: Available` are up for grabs.
- `Status: Accepted` are already taken for a PR.
- `Status: Review Needed` are up to review by a core contributor.
- `Status: Revision Needed` must be refactored.
- `Status: Abandoned` will not be continued at this time.
- `Status: Blocked` are stuck for some reason.

When in doubt, ask a core contributor by mentioning us on the issue.

**Documentation** is almost always a great place to start contributing to a new project. This project is an Open Source, community-driven project, so providing and maintaining quality documentation is one of our most important jobs. 

**Refactoring** code, or improving the code without modifying the behaviour, is an area that can probably be done based on intuition and may not require much communication to be merged. Generally speaking, you can rely on existing tests to ensure that your refactoring doesn't introduce any unexpected behaviour. However, you might be asked to write a regression test if the area you've refactored isn't well covered.

**Fixing bugs** may also not require a lot of communication, but it's always better to let us know what you're working on! Please surround bug fixes with ample tests; bugs are magnets for other bugs. Write tests around bugs!

**Building features** is the area that will require the most communication and/or negotiation. Every feature is subjective and open for debate. If your feature involves user-facing design changes, please provide a mockup first so we can all get on the same page. As always, when in doubt, ask!

## How to contribute
1. Fork the project and clone it to your local machine.
2. Create a branch with your GitHub username as a prefix and the ID of the issue as a suffix, for example: `git checkout -b USERNAME/that-new-feature-1234` or `git checkout -b USERNAME/fixing-that-bug-1234` where `USERNAME` should be replaced by your username and `1234` is the ID of the issue tied to your pull request. If there is no issue, you can leave the number out.
3. Code and commit your changes. Bonus points if you write a [good commit message](https://chris.beams.io/posts/git-commit/): `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin USERNAME/that-new-feature-1234`
5. Create a pull request for your branch 🎉

## Contribution guidelines
### Create an issue
Nobody's perfect. Something doesn't work? Something could be done better? Check to see if the issue already exists, and if it does, leave a comment to get our attention! If the issue doesn't already exist, feel free to create a new one. A core contributor will triage incoming issues.

_Please note: core contributor may update the title of an issue to more accurately reflect the request/bug._

### Please include tests
Some existing code may be poorly written or untested, so we must have more scrutiny going forward.

### Create a pull request
- Try to keep the pull requests small. A pull request should try its very best to address only a single concern.
- Make sure all tests pass and add additional tests for the code you submit.
- Document your reasoning behind the changes. Explain why you wrote the code in the way you did. The code should explain what it does.
- If there's an existing issue related to the pull request, reference to it by adding something like `References/Closes/Fixes/Resolves #305`, where 305 is the issue number. [More info here](https://github.com/blog/1506-closing-issues-via-pull-requests).
- Please fill out the PR Template when making a PR.
- All commits in a pull request will be squashed when merged, but when your PR is approved and passes our CI, it will be live on production!

_Please note: a core contributor may close your PR if it has gone stale or if we don't plan to merge the code._

### Pull requests reviews and "force pushing"
After you submit your pull request (PR), one of the core contributors will likely do a review of the code accepting it or giving feedback.

If feedback or suggestions are provided, any changes on your part should happenin separate commits added to the existing ones.

Force pushing, though understandable for reasons of wanting to keep the history clean has some drawbacks:

- it removes the review history of the code
- forces the reviewer to start from scratch when adding possible further comments

PRs will be squashed and merged into master, so there's no need to use force push.

Please avoid force pushing unless you need to rebase with the master branch.

## The bottom line
We are all humans trying to work together to improve the community. Always be kind and appreciate the need for tradeoffs. ❤️
