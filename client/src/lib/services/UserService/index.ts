import {
  UserAvatarUpdateDocument,
  UserRegisterDocument,
  UserUpdateDocument,
} from '$lib/graphql/schema';

import type { Client } from '@urql/core';
import type {
  CurrentUserFragment,
  UserErrorCode,
  UserRegisterInput,
  UserUpdateInput,
} from '$lib/graphql/schema';
import { GraphQLError } from '$lib/utils/graphql';

export class UserError extends GraphQLError<UserErrorCode> {}

export class UserService {
  static async userRegister(
    urqlClient: Client,
    input: UserRegisterInput,
  ): Promise<CurrentUserFragment> {
    const response = await urqlClient
      .mutation(
        UserRegisterDocument,
        {
          input,
        },
        {
          requestPolicy: 'network-only', // We dont want to cache this request
        },
      )
      .toPromise();

    if (response?.error || response?.data?.userRegister?.error) {
      if (response?.data?.userRegister?.error) {
        const error: UserError = response.data.userRegister.error;

        throw UserError.new(error.code, error.message);
      }

      throw response?.error;
    }

    return response.data.userRegister.user;
  }

  static async userAvatarUpdate(
    urqlClient: Client,
    file: File,
  ): Promise<CurrentUserFragment> {
    const response = await urqlClient
      .mutation(UserAvatarUpdateDocument, {
        file,
      })
      .toPromise();

    if (response?.error || response?.data?.userAvatarUpdate?.error) {
      if (response?.data?.userAvatarUpdate?.error) {
        const error: UserError = response.data.userAvatarUpdate.error;

        throw UserError.new(error.code, error.message);
      }

      throw response?.error;
    }

    return response.data.userAvatarUpdate.user;
  }

  static async userUpdate(
    urqlClient: Client,
    input: UserUpdateInput,
  ): Promise<CurrentUserFragment> {
    const response = await urqlClient
      .mutation(UserUpdateDocument, {
        input,
      })
      .toPromise();

    if (response.error) {
      throw new Error(response.error.message);
    }

    if (response.data?.userUpdate?.user) {
      return response.data?.userUpdate?.user;
    }

    const { code, message } = response.data?.userUpdate?.error || {
      code: 'UNKNOWN',
      message: 'Unknown error',
    };

    throw UserError.new(code, message);
  }
}
