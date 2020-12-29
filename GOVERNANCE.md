# jstime Project Governance

For the curent list of Collaborators see the project [README](./README.md).

This governance document is heavily inspired by the
[governance of the Node.js project](https://github.com/nodejs/node/blob/master/GOVERNANCE.md).

## Collaborators

jstime Collaborators maintain the [jstime/jstime][] repository. The GitHub team for
jstime Collaborators is @jstime/collaborators. Collaborators have:

* Commit access to the [jstime/jstime][] repository
* Ability to nominate, and vote for, the position of chair

Both Collaborators and non-Collaborators may propose changes to the jstime source
code. The mechanism to propose such a change is a GitHub pull request. Collaborators
review and merge (land) pull requests.

Pull Request authored by non-collaborators require at least one approval from a 
Collaborator. Approving a pull request indicates that the  Collaborator accepts 
responsibility for the change.

Pull Requests authored by a collabortor do not require review, but it is 
encouraged. If no review is given the collaborator should document the reason
for landing without review.

If a Collaborator opposes a proposed change, then the change cannot land unless
consensus to land can be reached or the chair determines there is
[rough consensus][] to move forward.

See:

* [List of Collaborators](./README.md#current-project-team-members)

### Collaborator Activies

* Helping users and novice contributors
* Contributing code and documentation changes that improve the project
* Reviewing and commenting on issues and pull requests
* Merging pull requests

## Chair

The chair has a sole additional responsibility beyond other collaborators, to
determine when rough consensus has been reached. It is imperative that the chair
remain impartial and fair in this decision making process. Collaborators have the
ability to nominate a new chair at any time who can be ratified either through
rough consensus or a two-thirds super majority vote.

## Collaborator nominations

Existing Collaborators can nominate someone to become a Collaborator.

To nominate a new Collaborator, start a private thread in the
[@jstime/collaborators team discussions](https://github.com/orgs/jstime/teams/collaborators). Include a short summary of why you would like to add them as a collaborator.

The nomination passes if no Collaborators oppose it after one week. Otherwise, the nomination fails.

## Consensus seeking process

The jstime project follows a [rough consensus][] decision-making model. The IETF
have an excellent [RFC on rough consensus](https://tools.ietf.org/html/rfc7282).

## Changes to Governance

Any change to governance must be open for at least 7 days to give all collaborators
a chance to approve. Changes to governance must either have no-objections or pass
via a two-thirds super majority vote of the collaborators.

[jstime/jstime]: https://github.com/jstime/jstime
[rough consensus]: https://en.wikipedia.org/wiki/Rough_consensus
