#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct
{
	char zero[1024];
	char *ones[20];
	char *tens[10];
	char *hundreds[8];
} digits;

struct
{
	char *out_range_err;
	char *nan_err[2];
	char *insert_num;
} UI;

int load_lang(const char *file_name)
{
	FILE *lang_fp;
	if (strcmp(file_name, "DEFAULT") == 0)
		lang_fp = fopen("./langs/italian.txt", "r");
	else
		lang_fp = fopen(file_name, "r");
	if (lang_fp == NULL)
	{
		fprintf(stderr, "File %s does not exist!\n", file_name);
		return -1;
	}

	char *buffer = malloc(2048);
	while (fgets(buffer, 2048, lang_fp))
	{
		char *var_name = malloc(strlen(buffer)), *value = malloc(strlen(buffer));
		int index = 0;
		sscanf(buffer, "%s %i = \"%[^\"]", var_name, &index, value);
		if (strcmp(var_name, "zero") == 0)
			sprintf(digits.zero, "%s", value);
		else if (strcmp(var_name, "ones") == 0)
		{
			digits.ones[index] = malloc(strlen(value));
			sprintf(digits.ones[index], "%s ", value);
		}
		else if (strcmp(var_name, "tens") == 0)
		{
			digits.tens[index] = malloc(strlen(value));
			sprintf(digits.tens[index], "%s ", value);
		}
		else if (strcmp(var_name, "hundreds") == 0)
		{
			digits.hundreds[index] = malloc(strlen(value));
			sprintf(digits.hundreds[index], "%s ", value);
		}
		else if (strcmp(var_name, "out_range_err") == 0)
		{
			UI.out_range_err = malloc(strlen(value));
			sprintf(UI.out_range_err, "%s ", value);
		}
		else if (strcmp(var_name, "nan_err") == 0)
		{
			UI.nan_err[index] = malloc(strlen(value));
			sprintf(UI.nan_err[index], "%s ", value);
		}
		else if (strcmp(var_name, "insert_num") == 0)
		{
			UI.insert_num = malloc(strlen(value));
			sprintf(UI.insert_num, "%s ", value);
		}
	}
	free(buffer);
	fclose(lang_fp);
	return 0;
}

char *digit_to_word(int num, char *plus_str)
{
	char *str = malloc(2048);
	if (num > 19)
	{
		strcat(str, digits.tens[num / 10]);
		strcat(str, digits.ones[num % 10]);
	}
	else
		sprintf(str, "%s", digits.ones[num]);
	if (num)
		strcat(str, plus_str);
	return str;
}

char *num_to_word(long int num)
{
	char *str = malloc(2048);
	sprintf(str, ""); // without this line the output will have some random characters in the beginning and I don't know why

	if (num == 0)
	{
		sprintf(str, "%s", digits.zero);
		return str;
	}

	// billions
	if ((num / 1000000000) % 1000000 == 1)
		sprintf(str, "%s", digits.hundreds[3]);
	else if ((num / 1000000000) % 1000000)
		strcat(str, digit_to_word(((num / 1000000000) % 1000000), digits.hundreds[7]));

	// millions
	if ((num / 1000000) % 1000 == 1)
		sprintf(str, "%s", digits.hundreds[2]);
	else if ((num / 1000000) % 1000)
		strcat(str, digit_to_word(((num / 1000000) % 1000), digits.hundreds[6]));

	// hundred thousands
	if ((num / 100000) % 10 == 1)
		sprintf(str, "%s", digits.hundreds[0]);
	else if ((num / 100000) % 10)
		strcat(str, digit_to_word(((num / 100000) % 10), digits.hundreds[4]));

	// thousands
	if ((num / 1000) % 100 == 1)
		sprintf(str, "%s", digits.hundreds[1]);
	else if ((num / 1000) % 100)
		strcat(str, digit_to_word(((num / 1000) % 100), digits.hundreds[5]));

	// hundreds
	if ((num / 100) % 10 == 1)
		sprintf(str, "%s", digits.hundreds[0]);
	else if ((num / 100) % 10)
		strcat(str, digit_to_word(((num / 100) % 10), digits.hundreds[4]));
	strcat(str, digit_to_word((num % 100), ""));

	return str;
}

int main(int argc, char **argv)
{
	char *lang_fn = malloc(1024);
	if (argc == 1)
		sprintf(lang_fn, "DEFAULT");
	else
		sprintf(lang_fn, "%s", argv[1]);
	if (load_lang(lang_fn) != 0)
		return -1;
	free(lang_fn);
	fprintf(stdout, "%s", UI.insert_num);

	long int num = 0;
	char *user_input = malloc(2048);
	fscanf(stdin, "%s", user_input);
	if (sscanf(user_input, "%li", &num) == 0)
	{
		fprintf(stderr, "%s\"%s\" %s", UI.nan_err[0], user_input, UI.nan_err[1]);
		return -1;
	}
	if (num > 1109999999)
		fprintf(stderr, "\n\033[0;31m%s\n\n\033[0m", UI.out_range_err);
	free(user_input);
	printf("%s\n", num_to_word(num));
	return 0;
}
